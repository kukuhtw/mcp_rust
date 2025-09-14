// backend/src/handlers/settings.rs
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiSettings {
    pub model: String,
    pub temperature: f32,
    pub top_p: f32,
    pub system_prompt: String,
    pub streaming: bool,
}

pub async fn get_settings() -> Json<UiSettings> {
    Json(UiSettings {
        model: "gpt-4o-mini".into(),
        temperature: 0.2,
        top_p: 0.9,
        system_prompt: "You are an MCP intent router...".into(),
        streaming: true,
    })
}

pub async fn update_settings(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<UiSettings>,
) -> Json<UiSettings> {
    // PoC: simpan ke DB kalau mau; sekarang echo balik
    Json(payload)
}
