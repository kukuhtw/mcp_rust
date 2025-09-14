// backend/src/handlers/gitlab_ci.rs
// backend/src/handlers/gitlab_ci.rs
use crate::router::Range;
use crate::util::now_gmt8;
use axum::{extract::Query, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct GitlabCIDummy {
    adapter: &'static str,
    project: &'static str,
    branch: &'static str,
    status: &'static str,         // success | failed
    checked_at: String,
    failed_tests: Vec<&'static str>,
}

pub async fn get_gitlab_ci(Query(_q): Query<Range>) -> Json<GitlabCIDummy> {
    Json(GitlabCIDummy {
        adapter: "gitlab_ci",
        project: "ticketing-backend",
        branch: "main",
        status: "success", // dummy: CI terakhir sukses
        checked_at: now_gmt8().to_rfc3339(),
        failed_tests: vec![], // kosong kalau sukses
    })
}
