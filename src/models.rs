use crate::{errors, fs::parse::string_to_f64};
use serde::Deserialize;
use std::str::FromStr;

// TODO: <symbol>@ticker
// 24hr rolling window ticker statistics for a single symbol.
// These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs.

// TODO: <symbol>@ticker_<window_size>
// Rolling window ticker statistics for a single symbol, computed over multiple windows.

// TODO: <symbol>@avgPrice
// Average price streams push changes in the average price over a fixed time interval.

// TODO: <symbol>@depth OR <symbol>@depth@100ms
// Order book price and quantity depth updates used to locally manage an order book.

// TODO: How to manage a local order book correctly

// Deserialize klines downloaded from data.binances.vision
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
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

impl PartialEq for HistoricalKlineEvent {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl Eq for HistoricalKlineEvent {}

impl PartialOrd for HistoricalKlineEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.t.cmp(&other.t))
    }
}

impl Ord for HistoricalKlineEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.t.cmp(&other.t)
    }
}

impl From<HistoricalKlineEvent> for KlineEvent {
    fn from(value: HistoricalKlineEvent) -> Self {
        let kline = Kline {
            t: value.t,
            T: value.T,
            s: String::new(),
            i: String::new(),
            f: 0,
            L: 0,
            o: value.o.to_string(),
            c: value.c,
            h: value.h.to_string(),
            l: value.l.to_string(),
            v: value.v.to_string(),
            n: value.n,
            x: true,
            q: value.q.to_string(),
            V: value.V.to_string(),
            Q: value.Q.to_string(),
            B: value.B.to_string(),
        };

        Self {
            e: String::new(),
            E: 0,
            s: String::new(),
            k: kline,
        }
    }
}

/// Deserialize klines received using binance websocket.
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct KlineEvent {
    e: String, //Event type
    E: u64,    //Event time
    s: String, //Symbol
    pub k: Kline,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct Kline {
    t: i64,    // Kline start time
    T: i64,    // Kline close time
    s: String, // Symbol
    i: String, // Interval (e.g. "1m")
    f: i64,    // First trade ID
    L: i64,    // Last trade ID
    o: String, // Open price
    #[serde(deserialize_with = "string_to_f64")]
    pub c: f64, // Close price
    h: String, // High price
    l: String, // Low price
    v: String, // Volume (base asset)
    n: u64,    // Number of trades
    x: bool,   // Is this kline closed?
    q: String, // Quote asset volume
    V: String, // Taker buy base asset volume
    Q: String, // Taker buy quote asset volume
    B: String, // Unused, can be ignored
}

/// Trait used to serialize string in the absence of field names
/// i.e. when serde can't be used.
pub trait FromDelimitedString<A>
where
    Self: Sized,
{
    fn parse_field<T: FromStr>(fields: &[&str], index: usize) -> Result<T, errors::Error>
    where
        T::Err: std::fmt::Display;

    fn from_delimited_string(line: A, delimiter: char) -> Result<Self, errors::Error>;
}

impl FromDelimitedString<&str> for HistoricalKlineEvent {
    fn parse_field<T: FromStr>(fields: &[&str], index: usize) -> Result<T, errors::Error>
    where
        T::Err: std::fmt::Display,
    {
        // TODO:: Update error handling?
        fields
            .get(index)
            .ok_or_else(|| errors::Error::Other(format!("Unable to retrieve index {}", index)))?
            .parse::<T>()
            .map_err(|e| errors::Error::Parse(format!("Failed to parse field {}: {}", index, e)))
    }

    fn from_delimited_string(line: &str, delimiter: char) -> Result<Self, errors::Error> {
        let splitted_line: Vec<&str> = line.split(delimiter).collect();
        if splitted_line.len() != 12 {
            return Err(format!("Length of line not equal to expected length").into());
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
