use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use redis::AsyncCommands;
use sqlx::{PgPool, Row};
use async_nats::Client as NatsClient;
use engine::EngineError;
use tracing::{info, error};
use tokio::time::{sleep, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchPayload {
    pub match_id: Uuid,
    pub project_id: Uuid,
    pub players: Vec<Uuid>,
}

pub struct GamingService {
    redis_client: redis::Client,
    db_pool: PgPool,
    nats_client: Arc<NatsClient>,
}

impl GamingService {
    pub fn new(redis_url: String, db_pool: PgPool, nats_client: Arc<NatsClient>) -> Self {
        let redis_client = redis::Client::open(redis_url).expect("Gaming: Failed to connect to Redis");
        Self {
            redis_client,
            db_pool,
            nats_client,
        }
    }

    /// Join the matchmaking queue using Redis Sorted Sets.
    pub async fn join_matchmaking(&self, project_id: Uuid, player_id: Uuid, rank: f64) -> Result<(), EngineError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await
            .map_err(|e| EngineError::Internal(anyhow::anyhow!("Redis connection failed: {}", e)))?;

        let key = format!("matchmaking:{}", project_id);
        let _: () = conn.zadd(key, player_id.to_string(), rank).await
            .map_err(|e| EngineError::Internal(anyhow::anyhow!("Redis ZADD failed: {}", e)))?;

        info!("Player {} joined matchmaking for project {}", player_id, project_id);
        Ok(())
    }

    /// Securely fetch tenant-isolated player inventory.
    pub async fn get_inventory(&self, project_id: Uuid, player_id: Uuid) -> Result<serde_json::Value, EngineError> {
        let rows = sqlx::query("SELECT item_id, quantity FROM player_inventory WHERE project_id = $1 AND player_id = $2")
            .bind(project_id)
            .bind(player_id)
            .fetch_all(&self.db_pool)
            .await
            .map_err(EngineError::Database)?;

        let items: Vec<_> = rows.into_iter().map(|row| {
            let item_id: String = row.try_get("item_id").unwrap_or_default();
            let quantity: i32 = row.try_get("quantity").unwrap_or(0);
            serde_json::json!({ "item_id": item_id, "quantity": quantity })
        }).collect();

        Ok(serde_json::json!(items))
    }

    /// Transactional update to prevent item duplication.
    pub async fn update_inventory(&self, project_id: Uuid, player_id: Uuid, item_id: String, delta: i32) -> Result<(), EngineError> {
        let mut tx = self.db_pool.begin().await.map_err(EngineError::Database)?;

        sqlx::query(
            "INSERT INTO player_inventory (project_id, player_id, item_id, quantity) 
             VALUES ($1, $2, $3, $4) 
             ON CONFLICT (project_id, player_id, item_id) 
             DO UPDATE SET quantity = player_inventory.quantity + $4"
        )
        .bind(project_id)
        .bind(player_id)
        .bind(&item_id)
        .bind(delta)
        .execute(&mut *tx)
        .await
        .map_err(EngineError::Database)?;

        tx.commit().await.map_err(EngineError::Database)?;
        Ok(())
    }

    pub fn spawn_matchmaker(&self) {
        let redis_client = self.redis_client.clone();
        let nats_client = self.nats_client.clone();

        tokio::spawn(async move {
            loop {
                if let Err(e) = Self::process_matchmaking_queues(&redis_client, &nats_client).await {
                    error!("Matchmaking loop error: {}", e);
                }
                sleep(Duration::from_secs(5)).await;
            }
        });
    }

    async fn process_matchmaking_queues(redis_client: &redis::Client, nats_client: &Arc<NatsClient>) -> Result<(), anyhow::Error> {
        let mut conn = redis_client.get_multiplexed_async_connection().await?;
        
        let keys: Vec<String> = conn.keys("matchmaking:*").await?;

        for key in keys {
            let players: Vec<String> = conn.zrange(&key, 0, 1).await?;
            
            if players.len() >= 2 {
                let project_id_str = key.trim_start_matches("matchmaking:");
                let project_id = Uuid::parse_str(project_id_str)?;
                
                let match_id = Uuid::new_v4();
                let player_uuids: Vec<Uuid> = players.iter()
                    .map(|p| Uuid::parse_str(p).unwrap())
                    .collect();

                let payload = MatchPayload {
                    match_id,
                    project_id,
                    players: player_uuids,
                };

                let members: Vec<&str> = players.iter().take(2).map(|s| s.as_str()).collect();
                let _: () = conn.zrem(&key, members).await?;

                let subject = format!("projects.{}.events.match.created", project_id);
                let data = serde_json::to_vec(&payload)?;
                nats_client.publish(subject, data.into()).await?;
                
                info!("Match {} created for project {}", match_id, project_id);
            }
        }
        Ok(())
    }
}
