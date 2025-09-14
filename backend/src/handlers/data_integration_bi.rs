// backend/src/handlers/data_integration_bi.rs

use crate::router::Range;
use crate::util::now_gmt8;
use axum::{extract::Query, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct DataIntegrationDummy {
    adapter: &'static str,
    pipeline: &'static str,
    checked_at: String,
}

pub async fn get_bi(Query(_q): Query<Range>) -> Json<DataIntegrationDummy> {
    Json(DataIntegrationDummy {
        adapter: "data_integration_bi",
        pipeline: "etl_ridership_to_bi",
        checked_at: now_gmt8().to_rfc3339(),
    })
}
