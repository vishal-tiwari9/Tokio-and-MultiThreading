use crate::types::{TradeTick, Signal};
use crate::orderbook::OrderBook;
use flume::Receiver;

pub async fn start_workers(rx: Receiver<TradeTick>, orderbook: std::sync::Arc<OrderBook>) {
    // Multithreading workers
    for i in 0..4 {
        let rx_clone = rx.clone();
        let orderbook_clone = orderbook.clone();
        
        tokio::spawn(async move {
            println!("Worker {} started", i);
            
            while let Ok(tick) = rx_clone.recv_async().await {
                orderbook_clone.update_price(&tick);
                
                // Simple signal logic
                if tick.price > 65000.0 {
                    println!("🚨 SIGNAL: BUY {} at {:.2}", tick.symbol, tick.price);
                } else if tick.price < 62000.0 {
                    println!("🚨 SIGNAL: SELL {} at {:.2}", tick.symbol, tick.price);
                }
            }
        });
    }
}