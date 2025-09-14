// backend/src/handlers/cloud_mon.rs

use crate::router::Range;
use crate::util::now_gmt8;
use axum::{extract::Query, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct CloudMonDummy {
    adapter: &'static str,
    resource: &'static str,
    checked_at: String,
}

pub async fn get_cloud_mon(Query(_q): Query<Range>) -> Json<CloudMonDummy> {
    Json(CloudMonDummy {
        adapter: "cloud_monitoring",
        resource: "payment-service@ecs",
        checked_at: now_gmt8().to_rfc3339(),
    })
}
