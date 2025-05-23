use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ChatMessage {
    #[serde(rename = "type")]
    msg_type: String,
    user: String,
    message: String,
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<ChatMessage>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    let (mut sender, mut receiver) = ws_stream.split();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = bcast_rx.recv().await {
            let json_msg = serde_json::to_string(&msg).unwrap_or_default();
            if sender.send(Message::text(json_msg)).await.is_err() {
                break;
            }
        }
        Ok::<_, Box<dyn Error + Send + Sync>>(())
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(msg) => {
                    if msg.is_text() {
                        let text = msg.as_text().unwrap_or("");
                        
                        // Try to parse as JSON first
                        if let Ok(mut chat_msg) = serde_json::from_str::<ChatMessage>(text) {
                            // Add sender info to user field
                            chat_msg.user = format!("{}@{}", chat_msg.user, addr);
                            println!("Received JSON from {}: {:?}", addr, chat_msg);
                            let _ = bcast_tx.send(chat_msg);
                        } else {
                            // Fallback to plain text (for backward compatibility)
                            let fallback_msg = ChatMessage {
                                msg_type: "message".to_string(),
                                user: format!("User@{}", addr),
                                message: text.to_string(),
                            };
                            println!("Received text from {}: {}", addr, text);
                            let _ = bcast_tx.send(fallback_msg);
                        }
                    } else if msg.is_close() {
                        break;
                    }
                }
                Err(e) => {
                    println!("Error receiving from {}: {}", addr, e);
                    break;
                }
            }
        }
        println!("Client {} disconnected", addr);
        Ok::<_, Box<dyn Error + Send + Sync>>(())
    });

    tokio::select! {
        result = &mut send_task => result??,
        result = &mut recv_task => result??,
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("WebSocket server listening on port 8000");
    println!("Compatible with both JSON (YewChat) and text clients");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {:?}", addr);
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            let (_, ws_stream) = ServerBuilder::new().accept(socket).await?;
            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}