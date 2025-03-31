use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};
use url::Url;

/// MEXC の Orderbook を購読する関数
pub async fn subscribe_orderbook(symbol: &str) {
    let url = format!("wss://wbs.mexc.com/ws");
    let url = Url::parse(&url).expect("Invalid WebSocket URL");

    let (mut ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to MEXC WebSocket");

    println!("✅ WebSocket connected to MEXC for symbol: {}", symbol);

    // サブスクライブメッセージ（MEXC公式仕様に従う）
    let sub_msg = format!(
        r#"{{"method": "SUBSCRIPTION", "params": ["spot@public.deals.v3.api@{}"], "id": 1}}"#,
        symbol
    );

    // メッセージ送信
    ws_stream
        .send(Message::Text(sub_msg))
        .await
        .expect("Failed to send subscription");

    println!("📡 Subscribed to orderbook: {}", symbol);

    // メッセージ受信ループ（最初の数件を表示）
    let mut count = 0;
    while let Some(Ok(msg)) = ws_stream.next().await {
        if let Message::Text(text) = msg {
            println!("📥 {}", text);
            count += 1;
            if count >= 5 {
                break;
            }
        }
    }

    println!("📴 Subscription finished (demo mode)");
}
