// backend/src/handlers/security_auth.rs
// backend/src/handlers/security_auth.rs

use crate::router::Range;
use crate::util::now_gmt8;
use axum::{extract::Query, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct SecurityAuthDummy {
    adapter: &'static str,
    service: &'static str,
    event: &'static str,
    error_rate: f32, // error rate dalam persen
    checked_at: String,
}

pub async fn get_security(Query(_q): Query<Range>) -> Json<SecurityAuthDummy> {
    Json(SecurityAuthDummy {
        adapter: "security_auth",
        service: "api-gateway-production",
        event: "failed_login",
        error_rate: 2.35, // dummy: 2.35% error rate sekarang
        checked_at: now_gmt8().to_rfc3339(),
    })
}
