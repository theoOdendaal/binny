use crate::errors::Error as ProjectError;
use chrono::offset::LocalResult;
use chrono::{DateTime, TimeZone, Utc};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};

// 1 January 2025
const TIMESTAMP_CHANGE_2025_MS: i64 = 1735689600000;

fn micro_seconds_to_datetime(t: &i64) -> LocalResult<DateTime<Utc>> {
    milli_seconds_to_datetime(&(t / 1000))
}

fn milli_seconds_to_datetime(t: &i64) -> LocalResult<DateTime<Utc>> {
    let secs = t / 1000;
    let nsecs = ((t % 1000) * 1_000_000) as u32;
    Utc.timestamp_opt(secs, nsecs)
}

// Binance changed the unit of their server timestamps
// in 2025 from milli- to micro- seconds.
pub fn binance_timestamp_to_datetime(t: &i64) -> Option<DateTime<Utc>> {
    let timestamp = if t > &TIMESTAMP_CHANGE_2025_MS {
        micro_seconds_to_datetime(t)
    } else {
        milli_seconds_to_datetime(t)
    };

    if let chrono::LocalResult::Single(datetime) = timestamp {
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
