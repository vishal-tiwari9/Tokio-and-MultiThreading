use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::StreamExt;
use crate::types::TradeTick;
use chrono::Utc;

pub async fn start_websocket_feed(tx: flume::Sender<TradeTick>) {
    // Multiple public streams
    let symbols = vec!["btcusdt", "ethusdt", "solusdt"];
    
    for symbol in symbols {
        let url = format!("wss://data-stream.binance.vision/ws/{}@ticker", symbol);
        let tx_clone = tx.clone();
        
        tokio::spawn(async move {
            println!("Connecting to: {}", url);
            
            match connect_async(&url).await {
                Ok((ws_stream, _)) => {
                    let (_, mut read) = ws_stream.split();
                    println!(" Now  Connected to {}", symbol.to_uppercase());
                    
                    while let Some(msg) = read.next().await {
                        if let Ok(Message::Text(text)) = msg {
                            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                                let tick = TradeTick {
                                    symbol: data["s"].as_str().unwrap_or("").to_string(),
                                    price: data["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                    quantity: data["v"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                                    timestamp: Utc::now(),
                                };
                                
                                let _ = tx_clone.send(tick);
                            }
                        }
                    }
                }
                Err(e) => println!("Connection failed for {}: {}", symbol, e),
            }
        });
    }
}