use chrono::TimeZone;

pub fn checked_string_to_f64(s: String) -> Option<f64> {
    let trimmed = s.as_str().trim_matches('"');
    trimmed.parse::<f64>().ok()
}

pub fn binance_timestamp_to_datetime(t: i64) -> Option<chrono::DateTime<chrono::Utc>> {
    let secs = t / 1000;
    let nsecs = ((t % 1000) * 1_000_000) as u32;

    if let chrono::LocalResult::Single(datetime) = chrono::Utc.timestamp_opt(secs, nsecs) {
        return Some(datetime);
    }
    None
}
