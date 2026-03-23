pub mod schema;

use rusqlite::{Connection, Result as SqliteResult};
use schema::{Command, NewCommand, UpdateCommand};
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;
use chrono::Utc;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Initialize database with tables
    pub fn new<P: AsRef<Path>>(path: P) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS commands (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                content TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                usage_count INTEGER DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        // Create index for sorting
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_commands_usage
             ON commands(usage_count DESC, updated_at DESC)",
            [],
        )?;

        Ok(Database { conn: Mutex::new(conn) })
    }

    /// Get all commands sorted by usage_count DESC, updated_at DESC
    pub fn get_all_commands(&self) -> SqliteResult<Vec<Command>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, content, description, created_at, updated_at, usage_count
             FROM commands
             ORDER BY usage_count DESC, updated_at DESC"
        )?;

        let commands = stmt.query_map([], |row| {
            Ok(Command {
                id: row.get(0)?,
                name: row.get(1)?,
                content: row.get(2)?,
                description: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                usage_count: row.get(6)?,
            })
        })?.collect::<SqliteResult<Vec<_>>>();

        commands
    }

    /// Create a new command with UUID and timestamps
    pub fn create_command(&self, new_cmd: NewCommand) -> SqliteResult<Command> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "INSERT INTO commands (id, name, content, description, created_at, updated_at, usage_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)",
            rusqlite::params![
                &id,
                &new_cmd.name,
                &new_cmd.content,
                &new_cmd.description,
                &now,
                &now,
            ],
        )?;

        Ok(Command {
            id,
            name: new_cmd.name,
            content: new_cmd.content,
            description: new_cmd.description,
            created_at: now.clone(),
            updated_at: now,
            usage_count: 0,
        })
    }

    /// Update existing command
    pub fn update_command(&self, update_cmd: UpdateCommand) -> SqliteResult<Command> {
        let now = Utc::now().to_rfc3339();
        {
            let conn = self.conn.lock().unwrap();
            conn.execute(
                "UPDATE commands
                 SET name = ?1, content = ?2, description = ?3, updated_at = ?4
                 WHERE id = ?5",
                rusqlite::params![
                    &update_cmd.name,
                    &update_cmd.content,
                    &update_cmd.description,
                    &now,
                    &update_cmd.id,
                ],
            )?;
        }

        // Fetch the updated command (separate lock scope)
        self.get_command_by_id(&update_cmd.id)?
            .ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)
    }

    /// Get a command by ID
    pub fn get_command_by_id(&self, id: &str) -> SqliteResult<Option<Command>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, content, description, created_at, updated_at, usage_count
             FROM commands WHERE id = ?1"
        )?;

        let mut commands = stmt.query_map([id], |row| {
            Ok(Command {
                id: row.get(0)?,
                name: row.get(1)?,
                content: row.get(2)?,
                description: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                usage_count: row.get(6)?,
            })
        })?;

        match commands.next() {
            Some(cmd) => Ok(Some(cmd?)),
            None => Ok(None),
        }
    }

    /// Delete command by id
    pub fn delete_command(&self, id: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM commands WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }

    /// Increment usage_count
    pub fn increment_usage(&self, id: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE commands SET usage_count = usage_count + 1, updated_at = ?1 WHERE id = ?2",
            rusqlite::params![
                Utc::now().to_rfc3339(),
                id,
            ],
        )?;
        Ok(())
    }

    /// Get setting by key
    pub fn get_setting(&self, key: &str) -> SqliteResult<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT value FROM settings WHERE key = ?1"
        )?;

        let mut values = stmt.query_map([key], |row| row.get(0))?;

        match values.next() {
            Some(v) => Ok(Some(v?)),
            None => Ok(None),
        }
    }

    /// Set setting by key
    pub fn set_setting(&self, key: &str, value: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            [key, value],
        )?;
        Ok(())
    }

    /// Export all commands as JSON
    pub fn export_to_json(&self) -> SqliteResult<String> {
        let commands = self.get_all_commands()?;
        serde_json::to_string(&commands)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))
    }

    /// Import commands from JSON
    pub fn import_from_json(&self, json: &str) -> SqliteResult<usize> {
        let commands: Vec<NewCommand> = serde_json::from_str(json)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;

        let mut count = 0;
        for cmd in commands {
            self.create_command(cmd)?;
            count += 1;
        }

        Ok(count)
    }

    /// Import commands from JSON (full format with all fields)
    pub fn import_commands(&self, commands: Vec<Command>) -> SqliteResult<usize> {
        let conn = self.conn.lock().unwrap();
        let mut count = 0;
        for cmd in commands {
            conn.execute(
                "INSERT OR REPLACE INTO commands (id, name, content, description, created_at, updated_at, usage_count)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![
                    &cmd.id,
                    &cmd.name,
                    &cmd.content,
                    &cmd.description,
                    &cmd.created_at,
                    &cmd.updated_at,
                    cmd.usage_count,
                ],
            )?;
            count += 1;
        }
        Ok(count)
    }

    /// Search commands by name or content
    pub fn search_commands(&self, query: &str) -> SqliteResult<Vec<Command>> {
        let pattern = format!("%{}%", query);
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, content, description, created_at, updated_at, usage_count
             FROM commands
             WHERE name LIKE ?1 OR content LIKE ?1 OR description LIKE ?1
             ORDER BY usage_count DESC, updated_at DESC"
        )?;

        let commands = stmt.query_map([&pattern], |row| {
            Ok(Command {
                id: row.get(0)?,
                name: row.get(1)?,
                content: row.get(2)?,
                description: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                usage_count: row.get(6)?,
            })
        })?.collect::<SqliteResult<Vec<_>>>();

        commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn get_test_db_path() -> PathBuf {
        std::env::temp_dir().join("test_commands.db")
    }

    fn setup_test_db() -> Database {
        let path = get_test_db_path();
        // Remove old test db if exists
        let _ = std::fs::remove_file(&path);
        Database::new(&path).unwrap()
    }

    #[test]
    fn test_create_and_get_command() {
        let db = setup_test_db();

        let new_cmd = NewCommand {
            name: "Test Command".to_string(),
            content: "echo hello".to_string(),
            description: Some("A test command".to_string()),
        };

        let created = db.create_command(new_cmd).unwrap();
        assert_eq!(created.name, "Test Command");
        assert_eq!(created.usage_count, 0);

        let all = db.get_all_commands().unwrap();
        assert_eq!(all.len(), 1);
    }

    #[test]
    fn test_update_command() {
        let db = setup_test_db();

        let new_cmd = NewCommand {
            name: "Original".to_string(),
            content: "original content".to_string(),
            description: None,
        };

        let created = db.create_command(new_cmd).unwrap();

        let update_cmd = UpdateCommand {
            id: created.id.clone(),
            name: "Updated".to_string(),
            content: "updated content".to_string(),
            description: Some("Now has description".to_string()),
        };

        let updated = db.update_command(update_cmd).unwrap();
        assert_eq!(updated.name, "Updated");
        assert!(updated.description.is_some());
    }

    #[test]
    fn test_delete_command() {
        let db = setup_test_db();

        let new_cmd = NewCommand {
            name: "To Delete".to_string(),
            content: "content".to_string(),
            description: None,
        };

        let created = db.create_command(new_cmd).unwrap();
        assert_eq!(db.get_all_commands().unwrap().len(), 1);

        db.delete_command(&created.id).unwrap();
        assert_eq!(db.get_all_commands().unwrap().len(), 0);
    }

    #[test]
    fn test_increment_usage() {
        let db = setup_test_db();

        let new_cmd = NewCommand {
            name: "Test".to_string(),
            content: "content".to_string(),
            description: None,
        };

        let created = db.create_command(new_cmd).unwrap();
        assert_eq!(created.usage_count, 0);

        db.increment_usage(&created.id).unwrap();

        let fetched = db.get_command_by_id(&created.id).unwrap().unwrap();
        assert_eq!(fetched.usage_count, 1);
    }

    #[test]
    fn test_settings() {
        let db = setup_test_db();

        assert!(db.get_setting("test_key").unwrap().is_none());

        db.set_setting("test_key", "test_value").unwrap();
        assert_eq!(db.get_setting("test_key").unwrap(), Some("test_value".to_string()));
    }

    #[test]
    fn test_export_import() {
        let db = setup_test_db();

        let new_cmd = NewCommand {
            name: "Export Test".to_string(),
            content: "content".to_string(),
            description: Some("desc".to_string()),
        };

        db.create_command(new_cmd).unwrap();

        let json = db.export_to_json().unwrap();
        assert!(json.contains("Export Test"));
    }
}