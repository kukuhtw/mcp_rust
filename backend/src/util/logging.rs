// backend/src/util/logging.rs

use std::{env, fs, path::PathBuf, sync::OnceLock, time::SystemTime};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{daily, hourly};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

static FILE_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

fn default_filter() -> String {
    "smrt_mcp_backend=debug,axum=info,tower_http=info,sqlx=warn".to_string()
}

fn ensure_dir(p: &PathBuf) {
    let _ = fs::create_dir_all(p);
}

pub fn init_tracing() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter()));

    let log_dir = env::var("LOG_DIR").unwrap_or_else(|_| "logs".to_string());
    let roll = env::var("LOG_ROLL").unwrap_or_else(|_| "daily".to_string()); // "daily" | "hourly"
    let file = env::var("LOG_FILE").unwrap_or_else(|_| "backend.log".to_string());

    // warning fix: not mutable
    let dir = PathBuf::from(&log_dir);
    ensure_dir(&dir);

    let appender = match roll.as_str() {
        "hourly" => hourly(&dir, &file),
        _ => daily(&dir, &file),
    };

    // panggil fungsi fully-qualified
    let (nb_writer, guard) = tracing_appender::non_blocking(appender);
    let _ = FILE_GUARD.set(guard);

    let console_layer = fmt::layer()
        .with_target(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_ansi(true)
        .compact();

    let file_layer = fmt::layer()
        .with_writer(nb_writer)
        .with_target(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_ansi(false)
        .compact();

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    tracing::info!(
        roll = %roll,
        dir = %dir.display(),
        file = %file,
        ts = ?SystemTime::now(),
        "📝 tracing initialized (console + rolling file)"
    );
}
