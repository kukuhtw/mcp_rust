// backend/src/util/time.rs
use chrono::{DateTime, FixedOffset, Utc};

/// Waktu sekarang di GMT+8 (mis. Asia/Singapore)
pub fn now_gmt8() -> DateTime<FixedOffset> {
    let now_utc = Utc::now();
    let tz = FixedOffset::east_opt(8 * 3600).expect("valid +08:00 offset");
    now_utc.with_timezone(&tz)
}
