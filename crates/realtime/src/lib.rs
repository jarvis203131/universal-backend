use async_nats::jetstream;
use axum::extract::ws::{Message, WebSocket};
use engine::EngineError;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct RealtimeFrame {
    pub action: String,
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

pub struct RealtimeManager {
    nats_client: async_nats::Client,
    js: jetstream::Context,
}

impl RealtimeManager {
    pub async fn new(nats_url: &str) -> Result<Self, EngineError> {
        let client = async_nats::connect(nats_url)
            .await
            .map_err(|e| EngineError::Realtime(format!("NATS connection failed: {}", e)))?;
        
        let js = jetstream::new(client.clone());
        
        Ok(Self {
            nats_client: client,
            js,
        })
    }

    pub async fn handle_session(
        self: Arc<Self>,
        mut socket: WebSocket,
        project_id: Uuid,
    ) -> Result<(), EngineError> {
        let (mut ws_tx, mut ws_rx) = socket.split();
        let (msg_tx, mut msg_rx) = mpsc::channel::<Message>(100);

        let pipe_task = tokio::spawn(async move {
            while let Some(msg) = msg_rx.recv().await {
                if ws_tx.send(msg).await.is_err() {
                    break;
                }
            }
        });

        let mut subscriptions = Vec::new();

        while let Some(Ok(msg)) = ws_rx.next().await {
            if let Message::Text(text) = msg {
                if let Ok(frame) = serde_json::from_str::<RealtimeFrame>(&text) {
                    match frame.action.as_str() {
                        "subscribe" => {
                            let channel = frame.channel.clone();
                            let subject = format!("projects.{}.channels.{}", project_id, channel);
                            
                            let msg_tx_clone = msg_tx.clone();
                            let nats_client = self.nats_client.clone();
                            
                            let sub_task = tokio::spawn(async move {
                                if let Ok(mut subscriber) = nats_client.subscribe(subject).await {
                                    while let Some(nats_msg) = subscriber.next().await {
                                        let payload = String::from_utf8_lossy(&nats_msg.payload);
                                        if msg_tx_clone.send(Message::Text(payload.to_string())).await.is_err() {
                                            break;
                                        }
                                    }
                                }
                            });
                            subscriptions.push(sub_task);
                        }
                        _ => {
                            let _ = msg_tx.send(Message::Text(
                                r#"{"error": "Unsupported action"}"#.to_string()
                            )).await;
                        }
                    }
                }
            }
        }

        pipe_task.abort();
        for sub in subscriptions {
            sub.abort();
        }

        Ok(())
    }
}
