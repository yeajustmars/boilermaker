use chrono::{DateTime, Utc};
use color_eyre::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp_millis() -> Result<i32> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i32)
}

pub fn current_timestamp_seconds() -> Result<i32> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i32)
}

pub fn timestamp_to_iso8601(timestamp: i64) -> String {
    let datetime =
        DateTime::<Utc>::from(UNIX_EPOCH + std::time::Duration::from_secs(timestamp as u64));
    datetime.to_rfc3339()
}
