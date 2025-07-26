use serde::Deserialize;

static BASE_END_POINT: &str = "wss://stream.binance.com:9443/ws";

// Order book price and quantity depth updates used to locally manage an order book.

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct DiffDepthStream {
    pub e: String,           // Event type
    pub E: u64,              // Event time (milliseconds since epoch)
    pub s: String,           // Symbol
    pub U: u64,              // First update ID in event
    pub u: u64,              // Final update ID in event
    pub b: Vec<[String; 2]>, // Bids: [price, quantity]
    pub a: Vec<[String; 2]>, // Asks: [price, quantity]
}
