use rhai::{Engine, Scope};
use std::sync::Arc;
use tokio::time::{timeout, Duration};
use engine::EngineError;
use common::SystemEvent;
use async_nats::Client as NatsClient;
use anyhow::anyhow;

pub struct FunctionRuntime {
    nats_client: Arc<NatsClient>,
}

impl FunctionRuntime {
    pub fn new(nats_client: Arc<NatsClient>) -> Self {
        Self { nats_client }
    }

    pub async fn execute_function(
        &self,
        js_code: &str,
        event: &SystemEvent,
        project_id: uuid::Uuid,
    ) -> Result<serde_json::Value, EngineError> {
        let nats_client = self.nats_client.clone();
        let code = js_code.to_string();
        let pid = project_id;

        let result = timeout(Duration::from_millis(500), async move {
            tokio::task::spawn_blocking(move || {
                // Instantiate engine inside the task to satisfy Send bounds and ensure isolation
                let mut engine = Engine::new();
                
                let nats_inner = nats_client.clone();
                let pid_inner = pid;
                
                engine.register_fn("publishEvent", move |channel: String, payload: String| {
                    let nats = nats_inner.clone();
                    let event = SystemEvent {
                        event_id: uuid::Uuid::new_v4(),
                        project_id: pid_inner,
                        event_type: channel,
                        timestamp: chrono::Utc::now(),
                        payload: serde_json::from_str(&payload).unwrap_or(serde_json::Value::Null),
                    };

                    tokio::spawn(async move {
                        let subject = format!("projects.{}.events.{}", event.project_id, event.event_type);
                        let data = serde_json::to_vec(&event).unwrap_or_default();
                        let _ = nats.publish(subject, data.into()).await;
                    });
                    
                    true
                });

                let mut scope = Scope::new();
                scope.push("project_id", pid.to_string());
                
                let result = engine.eval_with_scope::<String>(&mut scope, &code)
                    .map_err(|e| EngineError::Internal(anyhow!("Execution Error: {}", e)))?;
                
                Ok(serde_json::json!(result))
            }).await.map_err(|e| EngineError::Internal(anyhow!("Task Join Error: {}", e)))?
        }).await;

        match result {
            Ok(Ok(val)) => Ok(val),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(EngineError::Internal(anyhow!("Function execution timed out after 500ms"))),
        }
    }
}
