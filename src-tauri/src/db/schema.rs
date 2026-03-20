use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: String,
    pub name: String,
    pub content: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub usage_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCommand {
    pub name: String,
    pub content: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCommand {
    pub id: String,
    pub name: String,
    pub content: String,
    pub description: Option<String>,
}