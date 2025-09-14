// backend/src/handlers/incident_metrics.rs

// backend/src/handlers/incident_metrics.rs

use crate::router::Range;
use crate::util::now_gmt8;
use axum::{extract::Query, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ReleaseDuration {
    release: &'static str,
    staging_minutes: u32,
    production_minutes: u32,
}

#[derive(Serialize)]
pub struct IncidentMetricsDummy {
    adapter: &'static str,
    checked_at: String,
    releases: Vec<ReleaseDuration>,
}

pub async fn get_incident(Query(_q): Query<Range>) -> Json<IncidentMetricsDummy> {
    Json(IncidentMetricsDummy {
        adapter: "incident_metrics",
        checked_at: now_gmt8().to_rfc3339(),
        releases: vec![
            ReleaseDuration {
                release: "v1.2.3",
                staging_minutes: 12,
                production_minutes: 18,
            },
            ReleaseDuration {
                release: "v1.2.2",
                staging_minutes: 15,
                production_minutes: 20,
            },
            ReleaseDuration {
                release: "v1.2.1",
                staging_minutes: 10,
                production_minutes: 14,
            },
        ],
    })
}
