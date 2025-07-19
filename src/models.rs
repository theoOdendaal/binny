use std::str::FromStr;

use crate::errors::Error;
use serde::Deserialize;

// Deserialize klines downloaded from data.binances.vision
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct HistoricalKlineEvent {
    pub t: i64,    // Kline start time
    pub o: f64,    // Open price
    pub h: f64,    // High price
    pub l: f64,    // Low price
    pub c: f64,    // Close price
    pub v: f64,    // Volume (base asset)
    pub T: i64,    // Kline close time
    pub q: f64,    //Quote asset volume
    pub n: u64,    // Number of trades
    pub V: f64,    // Taker buy base asset volume
    pub Q: f64,    // Taker buy quote asset volume
    pub B: String, // Unused, can be ignored
}

/// Deserialize klines received using binance websocket.
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct KlineEvent {
    e: String, //Event type
    E: u64,    //Event time
    s: String, //Symbol
    k: Kline,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Kline {
    pub t: i64,    // Kline start time
    pub T: i64,    // Kline close time
    pub s: String, // Symbol
    pub i: String, // Interval (e.g. "1m")
    pub f: u64,    // First trade ID
    pub L: u64,    // Last trade ID
    pub o: String, // Open price
    pub c: String, // Close price
    pub h: String, // High price
    pub l: String, // Low price
    pub v: String, // Volume (base asset)
    pub n: u64,    // Number of trades
    pub x: bool,   // Is this kline closed?
    pub q: String, // Quote asset volume
    pub V: String, // Taker buy base asset volume
    pub Q: String, // Taker buy quote asset volume
    pub B: String, // Unused, can be ignored
}

/// Trait used to serialize string in the absence of field names
/// i.e. when serde can't be used.
pub trait FromDelimitedString<A>
where
    Self: Sized,
{
    fn parse_field<T: FromStr>(fields: &[&str], index: usize) -> Result<T, Error>
    where
        T::Err: std::fmt::Display;

    fn from_delimited_string(line: A, delimiter: char) -> Result<Self, Error>;
}

impl FromDelimitedString<&str> for HistoricalKlineEvent {
    fn parse_field<T: FromStr>(fields: &[&str], index: usize) -> Result<T, Error>
    where
        T::Err: std::fmt::Display,
    {
        // TODO Update error handling?
        fields
            .get(index)
            .ok_or(Error::Other(format!("Unable to retrieve index {}", index)))?
            .parse::<T>()
            .map_err(|e| Error::Parse(format!("Failed to parse field {}: {}", index, e)))
    }

    fn from_delimited_string(line: &str, delimiter: char) -> Result<Self, Error> {
        let splitted_line: Vec<&str> = line.split(delimiter).collect();
        if splitted_line.len() != 12 {
            return Err(Error::Other(format!(
                "Length of line not equal to expected length"
            )));
        }
        Ok(Self {
            t: Self::parse_field(&splitted_line, 0)?,
            o: Self::parse_field(&splitted_line, 1)?,
            h: Self::parse_field(&splitted_line, 2)?,
            l: Self::parse_field(&splitted_line, 3)?,
            c: Self::parse_field(&splitted_line, 4)?,
            v: Self::parse_field(&splitted_line, 5)?,
            T: Self::parse_field(&splitted_line, 6)?,
            q: Self::parse_field(&splitted_line, 7)?,
            n: Self::parse_field(&splitted_line, 8)?,
            V: Self::parse_field(&splitted_line, 9)?,
            Q: Self::parse_field(&splitted_line, 10)?,
            B: Self::parse_field(&splitted_line, 11)?,
        })
    }
}
