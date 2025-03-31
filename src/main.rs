mod exchange;

#[tokio::main]
async fn main() {
    println!("🚀 MEXC 板読みスタート！");
    
    // 任意のペアを指定（例: DOGE/USDT）
    exchange::mexc::subscribe_orderbook("dogeusdt").await;
}
