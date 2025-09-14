// backend/src/handlers/mobile_telemetry.rs

use crate::router::Range;
use crate::util::now_gmt8;
use axum::{extract::Query, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct MobileTelemetryDummy {
    adapter: &'static str,
    platform: &'static str,
    checked_at: String,
}

pub async fn get_mobile(Query(_q): Query<Range>) -> Json<MobileTelemetryDummy> {
    Json(MobileTelemetryDummy {
        adapter: "mobile_telemetry",
        platform: "android",
        checked_at: now_gmt8().to_rfc3339(),
    })
}
