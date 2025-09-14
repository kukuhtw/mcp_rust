// backend/src/handlers/db_perf.rs
use crate::router::Range;
use crate::util::now_gmt8;
use axum::{extract::Query, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct DbPerfDummy {
    adapter: &'static str,
    query: &'static str,
    checked_at: String,
}

pub async fn get_db_perf(Query(_q): Query<Range>) -> Json<DbPerfDummy> {
    Json(DbPerfDummy {
        adapter: "db_perf",
        query: "SELECT * FROM passenger_info WHERE ...",
        checked_at: now_gmt8().to_rfc3339(),
    })
}
