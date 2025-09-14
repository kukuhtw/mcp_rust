// backend/src/models.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatLog {
    pub id: i64,
    pub user_id: Option<String>,
    pub user_query: String,
    pub detected_intent: Option<String>,
    pub routed_endpoints: Option<serde_json::Value>,
    pub response_summary: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugTrace {
    pub id: i64,
    pub trace_id: String,
    pub phase: String,
    pub payload: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub id: i32,
    pub system_prompt: String,
    pub response_prompt: Option<String>,
    pub model: String,
    pub temperature: f32,
    pub top_p: f32,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
