use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub async fn run_public_demo() -> Result<()> {
    let (ws_stream, _) = connect_async("wss://www.deribit.com/ws/api/v2").await?;
    let (mut write, mut read) = ws_stream.split();

    let msg = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "public/subscribe",
        "params": {
            "channels": [
                "ticker.BTC-PERPETUAL.raw"
            ]
        }
    });

    write.send(Message::Text(msg.to_string().into())).await?;

    let mut count = 0usize;
    while let Some(message) = read.next().await {
        let message = message?;
        if let Message::Text(txt) = message {
            println!("{}", txt);
            count += 1;
            if count >= 3 {
                break;
            }
        }
    }

    Ok(())
}
