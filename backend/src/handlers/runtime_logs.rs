// backend/src/handlers/runtime_logs.rs

use axum::{extract::Query, Json};
use serde::Serialize;

use crate::router::Range;
use crate::util::now_gmt8;

#[derive(Serialize)]
pub struct LogLine {
    ts: String,
    level: &'static str,
    message: String,
}

#[derive(Serialize)]
pub struct RuntimeLogsResp {
    adapter: &'static str,
    service: String,
    container: String,
    checked_at: String,
    tz: String,
    logs: Vec<LogLine>,
}

/// GET /api/runtime-logs?service=payments&tz=Asia/Singapore&limit=5
pub async fn get_runtime_logs(Query(q): Query<Range>) -> Json<RuntimeLogsResp> {
    let tz = q.tz.as_deref().unwrap_or("Asia/Singapore").to_string();
    let service = q.service.clone().unwrap_or_else(|| "unknown".to_string());
    let limit = q.limit.unwrap_or(5).clamp(1, 200);

    let container: String = match service.as_str() {
        "payments" | "payment" => "payments-service".to_string(),
        "auth" | "auth-service" => "auth-service".to_string(),
        "orders" | "order" => "orders-service".to_string(),
        other => {
            if other == "unknown" {
                "unknown-service".to_string()
            } else {
                format!("{other}-service")
            }
        }
    };

    let mut logs = Vec::with_capacity(limit);
    for i in 0..limit {
        logs.push(LogLine {
            ts: now_gmt8().to_rfc3339(),
            level: match i % 5 {
                0 => "INFO",
                1 => "DEBUG",
                2 => "WARN",
                3 => "ERROR",
                _ => "INFO",
            },
            message: format!(
                "[{service}] line #{i} — synthetic log line for {container} (tz={tz})"
            ),
        });
    }

    Json(RuntimeLogsResp {
        adapter: "runtime_logs",
        service,
        container,
        checked_at: now_gmt8().to_rfc3339(),
        tz,
        logs,
    })
}
