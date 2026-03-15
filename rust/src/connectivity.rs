use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub async fn subscribe(url: &str, payload: Value, max_messages: usize) -> Result<Vec<String>> {
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    write.send(Message::Text(payload.to_string().into())).await?;

    let mut out = Vec::new();
    while let Some(message) = read.next().await {
        let message = message?;
        if let Message::Text(txt) = message {
            out.push(txt.to_string());
            if out.len() >= max_messages {
                break;
            }
        }
    }

    Ok(out)
}
