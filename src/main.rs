mod types;
mod market_data;
mod orderbook;
mod indicators;
mod worker;

use std::sync::Arc;
use flume;

#[tokio::main]
async fn main() {
    println!("🚀 Real-time Trading Engine Started (Public WebSockets)");
    println!("Listening to BTC, ETH, SOL live data...\n");

    // Channel for data flow
    let (tx, rx) = flume::unbounded();

    // Shared OrderBook
    let orderbook = Arc::new(orderbook::OrderBook::new());

    // Start WebSocket Feed
    tokio::spawn(market_data::start_websocket_feed(tx));

    // Start Workers (Multithreading)
    tokio::spawn(worker::start_workers(rx, orderbook));

    // Keep main thread alive
    tokio::signal::ctrl_c().await.unwrap();
    println!("\nShutting down gracefully...");
}