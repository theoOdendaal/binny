use crate::errors::Error as ProjectError;
use chrono::TimeZone;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};

pub fn binance_timestamp_to_datetime(t: &i64) -> Option<chrono::DateTime<chrono::Utc>> {
    let secs = t / 1000;
    let nsecs = ((t % 1000) * 1_000_000) as u32;

    if let chrono::LocalResult::Single(datetime) = chrono::Utc.timestamp_opt(secs, nsecs) {
        return Some(datetime);
    }
    None
}

pub fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.trim_matches('"')
        .parse::<f64>()
        .map_err(|_| D::Error::custom(ProjectError::Parse(String::from("Unable to parse to f64"))))
}
