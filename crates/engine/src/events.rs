use async_nats::jetstream::{self, Context};
use async_nats::Client;
use common::SystemEvent;
use std::sync::Arc;
use crate::errors::EngineError;

pub struct EventBus {
    client: Client,
    js: Context,
}

impl EventBus {
    pub async fn new(nats_url: &str) -> Result<Self, EngineError> {
        let client = async_nats::connect(nats_url).await
            .map_err(|e| EngineError::EventBus(format!("Failed to connect to NATS: {}", e)))?;
        
        let js = jetstream::new(client.clone());

        // Configure the durable stream ENGINE_EVENTS
        // Subjects: projects.*.events.*
        let stream_config = jetstream::stream::Config {
            name: "ENGINE_EVENTS".to_string(),
            subjects: vec!["projects.*.events.*".to_string()],
            ..Default::default()
        };

        js.create_stream(stream_config).await
            .map_err(|e| EngineError::EventBus(format!("Failed to create JetStream: {}", e)))?;

        Ok(Self { client, js })
    }

    pub async fn publish_event(&self, event: SystemEvent) -> Result<(), EngineError> {
        let subject = format!("projects.{}.events.{}", event.project_id, event.event_type);
        let payload = serde_json::to_vec(&event)
            .map_err(|e| EngineError::EventBus(format!("Serialization error: {}", e)))?;

        self.js.publish(subject, payload.into()).await
            .map_err(|e| EngineError::EventBus(format!("Publish error: {}", e)))?;

        Ok(())
    }
}

pub type SharedEventBus = Arc<EventBus>;
