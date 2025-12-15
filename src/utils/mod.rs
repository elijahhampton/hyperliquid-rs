use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current time in milliseconds.
#[allow(clippy::expect_used)]
pub fn current_time_millis() -> u64 {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX epoch!");

    dur.as_secs()
        .saturating_mul(1_000)
        .saturating_add(u64::from(dur.subsec_millis()))
}
