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
