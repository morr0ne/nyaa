use std::time::{SystemTime, UNIX_EPOCH};

/// Time since "1970-01-01 00:00:00 UTC" (Unix epoch) in seconds
pub fn since_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
