use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeTick {
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum Signal {
    Buy(String, f64),   // symbol, confidence
    Sell(String, f64),
    Hold,
}