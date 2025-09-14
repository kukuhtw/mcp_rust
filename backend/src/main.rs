// backend/src/main.rs
mod config;
mod db;
mod handlers;
mod intent; // jika ada
mod models;
mod router;
mod util;

mod fetch;
mod mcp;

use axum::{http, routing::get, Router};
use std::time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::Span;
use util::logging::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();

    let pool = db::init_db().await.expect("DB connection failed");

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .merge(router::app_routes(pool.clone()))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(|req: &http::Request<_>| {
                        let ua = req.headers().get(http::header::USER_AGENT).and_then(|v| v.to_str().ok()).unwrap_or("-");
                        tracing::info_span!("http", method=%req.method(), uri=%req.uri(), version=?req.version(), user_agent=%ua)
                    })
                    .on_request(|req: &http::Request<_>, _span: &Span| {
                        tracing::info!("➡️  request started: {} {}", req.method(), req.uri());
                    })
                    .on_response(|res: &http::Response<_>, latency: Duration, _span: &Span| {
                        tracing::info!(status=%res.status(), latency_ms=%latency.as_millis(), "✅ response finished");
                    })
                    .on_failure(|err, latency: Duration, _span: &Span| {
                        tracing::error!(%err, latency_ms=%latency.as_millis(), "💥 response failed");
                    })
            )
        );

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{port}");
    tracing::info!("🚀 SMRT MCP Backend on {}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
