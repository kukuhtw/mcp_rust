// backend/src/handlers/user_feedback.rs

use crate::router::Range;
use crate::util::now_gmt8;
use axum::{extract::Query, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserFeedbackDummy {
    adapter: &'static str,
    source: &'static str,
    checked_at: String,
}

pub async fn get_feedback(Query(_q): Query<Range>) -> Json<UserFeedbackDummy> {
    Json(UserFeedbackDummy {
        adapter: "user_feedback",
        source: "playstore",
        checked_at: now_gmt8().to_rfc3339(),
    })
}
