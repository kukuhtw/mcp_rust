// backend/src/handlers/observability.rs

use crate::router::Range;
use crate::util::now_gmt8;
use axum::{extract::Query, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ObservabilityDummy {
    adapter: &'static str,
    metric: &'static str,
    service: &'static str,
    window: &'static str,        // periode waktu
    avg_response_ms: u32,        // rata-rata response time dalam ms
    unresolved_tickets: u32,     // jumlah tiket belum selesai
    checked_at: String,
}

pub async fn get_observability(Query(_q): Query<Range>) -> Json<ObservabilityDummy> {
    Json(ObservabilityDummy {
        adapter: "observability",
        metric: "latency_p95",
        service: "orders-api",
        window: "last_24h",
        avg_response_ms: 243, // dummy: rata-rata 243 ms
        unresolved_tickets: 7,
        checked_at: now_gmt8().to_rfc3339(),
    })
}
