use crate::types::TradeTick;

pub fn calculate_simple_ma(prices: &[f64], period: usize) -> Option<f64> {
    if prices.len() < period {
        return None;
    }
    let sum: f64 = prices.iter().rev().take(period).sum();
    Some(sum / period as f64)
}