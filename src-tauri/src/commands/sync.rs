// src-tauri/src/commands/sync.rs
use crate::db::{Database, schema::Command};
use crate::DbState;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

const GIST_SCHEMA: &str = "Command-Repo-Sync";

// Data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncData {
    pub version: i32,
    pub schema: String,
    pub device_id: String,
    pub device_name: String,
    pub updated_at: String,
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub connected: bool,
    pub username: Option<String>,
    pub gist_id: Option<String>,
    pub device_name: Option<String>,
    pub last_sync: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub pushed: Option<usize>,
    pub pulled: Option<usize>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GitHubUser {
    login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Gist {
    id: String,
    description: Option<String>,
    files: std::collections::HashMap<String, GistFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GistFile {
    content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateGistRequest {
    description: String,
    public: bool,
    files: std::collections::HashMap<String, GistFileContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GistFileContent {
    content: String,
}

// Token management - store in database with encryption
fn save_token(db: &Database, token: &str) -> Result<(), String> {
    eprintln!("Attempting to save token to database...");
    db.set_setting("github_token", token)
        .map_err(|e: rusqlite::Error| format!("Failed to save token: {}", e))?;
    eprintln!("Token saved successfully to database");
    Ok(())
}

fn get_token(db: &Database) -> Result<Option<String>, String> {
    match db.get_setting("github_token") {
        Ok(Some(token)) => {
            eprintln!("Token retrieved successfully from database");
            Ok(Some(token))
        }
        Ok(None) => {
            eprintln!("No token found in database");
            Ok(None)
        }
        Err(e) => {
            eprintln!("Failed to get token from database: {}", e);
            Ok(None)
        }
    }
}

fn delete_token(db: &Database) -> Result<(), String> {
    db.set_setting("github_token", "")
        .map_err(|e: rusqlite::Error| e.to_string())
}

// Device ID
fn get_or_create_device_id(db: &Database) -> Result<String, String> {
    if let Some(id) = db.get_setting("device_id").map_err(|e: rusqlite::Error| e.to_string())? {
        return Ok(id);
    }

    let id = Uuid::new_v4().to_string();
    db.set_setting("device_id", &id).map_err(|e: rusqlite::Error| e.to_string())?;
    Ok(id)
}

// GitHub API helpers
async fn verify_token(token: &str) -> Result<String, String> {
    let client = Client::new();
    let response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "CommandRepo")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err("Invalid token or API error".to_string());
    }

    let user: GitHubUser = response.json().await.map_err(|e| e.to_string())?;
    Ok(user.login)
}

async fn discover_gist(token: &str) -> Result<Option<String>, String> {
    let client = Client::new();
    let response = client
        .get("https://api.github.com/gists")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "CommandRepo")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let gists: Vec<Gist> = response.json().await.map_err(|e| e.to_string())?;

    for gist in gists {
        if gist.description.as_deref() == Some(GIST_SCHEMA) {
            return Ok(Some(gist.id));
        }
    }

    Ok(None)
}

async fn create_gist(token: &str, data: &SyncData) -> Result<String, String> {
    let client = Client::new();
    let content = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;

    let mut files = std::collections::HashMap::new();
    files.insert(
        "commands.json".to_string(),
        GistFileContent { content },
    );

    let request = CreateGistRequest {
        description: GIST_SCHEMA.to_string(),
        public: false,
        files,
    };

    let response = client
        .post("https://api.github.com/gists")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "CommandRepo")
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Failed to create gist: {}", response.status()));
    }

    let gist: Gist = response.json().await.map_err(|e| e.to_string())?;
    Ok(gist.id)
}

async fn get_gist_content(token: &str, gist_id: &str) -> Result<SyncData, String> {
    let client = Client::new();
    let response = client
        .get(format!("https://api.github.com/gists/{}", gist_id))
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "CommandRepo")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Failed to get gist: {}", response.status()));
    }

    let gist: Gist = response.json().await.map_err(|e| e.to_string())?;

    let content = gist
        .files
        .get("commands.json")
        .and_then(|f| f.content.as_ref())
        .ok_or("No commands.json in gist")?;

    serde_json::from_str(content).map_err(|e| e.to_string())
}

async fn update_gist(token: &str, gist_id: &str, data: &SyncData) -> Result<(), String> {
    let client = Client::new();
    let content = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;

    let mut files = std::collections::HashMap::new();
    files.insert(
        "commands.json".to_string(),
        GistFileContent { content },
    );

    let response = client
        .patch(format!("https://api.github.com/gists/{}", gist_id))
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "CommandRepo")
        .json(&serde_json::json!({ "files": files }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Failed to update gist: {}", response.status()));
    }

    Ok(())
}

// Merge strategy: Last-Write-Wins
fn merge_commands(local: Vec<Command>, remote: Vec<Command>) -> Vec<Command> {
    let mut map: std::collections::HashMap<String, Command> = std::collections::HashMap::new();

    // Add local commands
    for cmd in local {
        map.insert(cmd.id.clone(), cmd);
    }

    // Merge with remote (remote wins if newer)
    for cmd in remote {
        if let Some(local_cmd) = map.get(&cmd.id) {
            // Parse timestamps for comparison
            let local_time = DateTime::parse_from_rfc3339(&local_cmd.updated_at);
            let remote_time = DateTime::parse_from_rfc3339(&cmd.updated_at);

            if let (Ok(lt), Ok(rt)) = (local_time, remote_time) {
                if rt > lt {
                    map.insert(cmd.id.clone(), cmd);
                }
            }
        } else {
            map.insert(cmd.id.clone(), cmd);
        }
    }

    map.values().cloned().collect()
}

// Tauri commands
#[tauri::command]
pub async fn get_sync_status(db: State<'_, DbState>) -> Result<SyncStatus, String> {
    let token = get_token(&db)?;
    let gist_id = db.get_setting("gist_id").map_err(|e: rusqlite::Error| e.to_string())?;
    let device_name = db.get_setting("device_name").map_err(|e: rusqlite::Error| e.to_string())?;
    let last_sync = db.get_setting("last_sync").map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(SyncStatus {
        connected: token.is_some() && gist_id.is_some(),
        username: None,
        gist_id,
        device_name,
        last_sync,
    })
}

#[tauri::command]
pub async fn connect_with_token(
    token: String,
    device_name: String,
    db: State<'_, DbState>,
) -> Result<SyncStatus, String> {
    // Verify token
    let username = verify_token(&token).await?;

    // Save token BEFORE any gist operations to prevent orphaned gists
    save_token(&db, &token)?;

    // Save device name
    db.set_setting("device_name", &device_name)
        .map_err(|e: rusqlite::Error| e.to_string())?;

    // Discover or create gist
    let gist_id = match discover_gist(&token).await? {
        Some(id) => {
            db.set_setting("gist_id", &id).map_err(|e: rusqlite::Error| e.to_string())?;
            id
        }
        None => {
            // Create new gist with current commands
            let device_id = get_or_create_device_id(&db)?;
            let commands = db.get_all_commands().map_err(|e: rusqlite::Error| e.to_string())?;

            let data = SyncData {
                version: 1,
                schema: GIST_SCHEMA.to_string(),
                device_id,
                device_name: device_name.clone(),
                updated_at: Utc::now().to_rfc3339(),
                commands,
            };

            let id = create_gist(&token, &data).await?;
            db.set_setting("gist_id", &id).map_err(|e: rusqlite::Error| e.to_string())?;
            id
        }
    };

    Ok(SyncStatus {
        connected: true,
        username: Some(username),
        gist_id: Some(gist_id),
        device_name: Some(device_name),
        last_sync: None,
    })
}

#[tauri::command]
pub async fn disconnect(db: State<'_, DbState>) -> Result<(), String> {
    delete_token(&db)?;
    db.set_setting("gist_id", "").map_err(|e: rusqlite::Error| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn push_to_gist(db: State<'_, DbState>) -> Result<SyncResult, String> {
    let token = get_token(&db)?.ok_or("Not connected")?;
    let gist_id = db.get_setting("gist_id").map_err(|e: rusqlite::Error| e.to_string())?
        .ok_or("No gist ID configured")?;
    let device_id = get_or_create_device_id(&db)?;
    let device_name = db.get_setting("device_name").map_err(|e: rusqlite::Error| e.to_string())?
        .unwrap_or_else(|| "Unknown".to_string());
    let commands = db.get_all_commands().map_err(|e: rusqlite::Error| e.to_string())?;

    let data = SyncData {
        version: 1,
        schema: GIST_SCHEMA.to_string(),
        device_id,
        device_name,
        updated_at: Utc::now().to_rfc3339(),
        commands: commands.clone(),
    };

    update_gist(&token, &gist_id, &data).await?;

    db.set_setting("last_sync", &Utc::now().to_rfc3339())
        .map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(SyncResult {
        pushed: Some(commands.len()),
        pulled: None,
        message: format!("成功推送 {} 条命令", commands.len()),
    })
}

#[tauri::command]
pub async fn pull_from_gist(db: State<'_, DbState>) -> Result<SyncResult, String> {
    let token = get_token(&db)?.ok_or("Not connected")?;
    let gist_id = db.get_setting("gist_id").map_err(|e: rusqlite::Error| e.to_string())?
        .ok_or("No gist ID configured")?;

    let remote_data = get_gist_content(&token, &gist_id).await?;
    let local_commands = db.get_all_commands().map_err(|e: rusqlite::Error| e.to_string())?;

    let merged = merge_commands(local_commands, remote_data.commands);

    let count = merged.len();
    db.import_commands(merged).map_err(|e: rusqlite::Error| e.to_string())?;

    db.set_setting("last_sync", &Utc::now().to_rfc3339())
        .map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(SyncResult {
        pushed: None,
        pulled: Some(count),
        message: format!("成功拉取并合并 {} 条命令", count),
    })
}

#[tauri::command]
pub async fn link_gist(gist_id: String, db: State<'_, DbState>) -> Result<(), String> {
    db.set_setting("gist_id", &gist_id).map_err(|e: rusqlite::Error| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn copy_gist_id(db: State<'_, DbState>) -> Result<String, String> {
    db.get_setting("gist_id").map_err(|e: rusqlite::Error| e.to_string())?
        .ok_or("No gist ID configured".to_string())
}