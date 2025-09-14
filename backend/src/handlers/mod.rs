// backend/src/handlers/mod.rs
pub mod chat; // <— supaya handlers::chat terlihat
pub mod cloud_mon;
pub mod data_integration_bi;
pub mod db_perf;
pub mod gitlab_ci;
pub mod incident_metrics;
pub mod mobile_telemetry;
pub mod observability;
pub mod runtime_logs;
pub mod security_auth;
pub mod settings;
pub mod user_feedback; // <-- tambahkan ini
                       // pub mod gitlab_ci; ...
                       // dst.
