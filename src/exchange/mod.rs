// src/exchange/mexc.rs

use std::time::Duration;
use tokio::time::sleep;

/// MEXCのオーダーブック購読関数（テスト版）
pub async fn subscribe_orderbook(pair: &str) {
    println!("🔁 Subscribing to MEXC orderbook for pair: {}", pair);

    // 擬似的な板読み処理
    loop {
        println!("📈 [MEXC] Orderbook update for {}", pair);
        sleep(Duration::from_secs(5)).await; // 5秒おきに更新（擬似）
    }
}

// src/exchange/mod.rs
pub mod mexc;
