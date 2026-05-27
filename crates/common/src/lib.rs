use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub event_id: Uuid,
    pub project_id: Uuid,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub payload: Value,
}

impl SystemEvent {
    pub fn new(project_id: Uuid, event_type: String, payload: Value) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            project_id,
            event_type,
            timestamp: Utc::now(),
            payload,
        }
    }
}
