use dashmap::DashMap;
use crate::types::TradeTick;

pub struct OrderBook {
    pub data: DashMap<String, f64>, // symbol -> latest price
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
        }
    }

    pub fn update_price(&self, tick: &TradeTick) {
        self.data.insert(tick.symbol.clone(), tick.price);
        println!("📊 OrderBook Updated: {} @ {:.2}", tick.symbol, tick.price);
    }
}