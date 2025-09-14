// backend/src/router.rs

use axum::{
    extract::Query,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::handlers;

#[derive(Debug, Deserialize, Clone)]
pub struct Range {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub tz: Option<String>,      // default Asia/Singapore
    pub service: Option<String>, // ← untuk runtime-logs
    pub limit: Option<usize>,    // ← optional
}

#[derive(Debug, Serialize)]
pub struct JoinResult {
    endpoint: String,
    data: serde_json::Value,
}

pub fn app_routes(pool: MySqlPool) -> Router {
    Router::new()
        .route("/api/gitlab-ci", get(handlers::gitlab_ci::get_gitlab_ci))
        .route(
            "/api/runtime-logs",
            get(handlers::runtime_logs::get_runtime_logs),
        )
        .route("/api/cloud-mon", get(handlers::cloud_mon::get_cloud_mon))
        .route("/api/db-perf", get(handlers::db_perf::get_db_perf))
        .route(
            "/api/observability",
            get(handlers::observability::get_observability),
        )
        .route(
            "/api/mobile-telemetry",
            get(handlers::mobile_telemetry::get_mobile),
        )
        .route(
            "/api/security-auth",
            get(handlers::security_auth::get_security),
        )
        .route(
            "/api/incident-metrics",
            get(handlers::incident_metrics::get_incident),
        )
        .route(
            "/api/user-feedback",
            get(handlers::user_feedback::get_feedback),
        )
        .route(
            "/api/data-integration-bi",
            get(handlers::data_integration_bi::get_bi),
        )
        .route("/api/test-join", get(test_join))
        .route(
            "/api/settings",
            get(handlers::settings::get_settings).post(handlers::settings::update_settings),
        )
        .route("/api/chat", post(handlers::chat::chat_handler))
        .route("/api/chat/stream", get(handlers::chat::chat_stream_handler))
        .route(
            "/internal/openai/ping",
            get(handlers::chat::openai_ping_handler),
        )
        .with_state(pool)
}

async fn test_join(Query(q): Query<Range>) -> Json<serde_json::Value> {
    let eps: Vec<String> = vec!["/api/gitlab-ci".into(), "/api/runtime-logs".into()];

    let mut results: Vec<JoinResult> = Vec::with_capacity(eps.len());
    for ep in eps {
        results.push(JoinResult {
            endpoint: ep.clone(),
            data: serde_json::json!({
                "ok": true,
                "endpoint": ep,
                "date_from": q.date_from,
                "date_to": q.date_to,
                "tz": q.tz.clone().unwrap_or_else(|| "Asia/Singapore".to_string()),
                "service": q.service,
                "limit": q.limit
            }),
        });
    }
    Json(serde_json::json!({ "results": results }))
}
