use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::{Client, primitives::ByteStream};
use std::time::Duration;
use async_trait::async_trait;
use engine::EngineError;

#[async_trait]
pub trait ObjectStorage: Send + Sync {
    async fn upload(&self, project_id: &str, bucket: &str, path: &str, data: Vec<u8>) -> Result<(), EngineError>;
    async fn sign_url(&self, project_id: &str, bucket: &str, path: &str, expires_in: Duration) -> Result<String, EngineError>;
}

pub struct S3Storage {
    client: Client,
    base_bucket: String,
}

impl S3Storage {
    pub async fn new(base_bucket: String) -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        Self {
            client,
            base_bucket,
        }
    }

    fn resolve_path(&self, project_id: &str, bucket: &str, path: &str) -> String {
        format!("projects/{}/{}/{}", project_id, bucket, path.trim_start_matches('/'))
    }
}

#[async_trait]
impl ObjectStorage for S3Storage {
    async fn upload(&self, project_id: &str, bucket: &str, path: &str, data: Vec<u8>) -> Result<(), EngineError> {
        let key = self.resolve_path(project_id, bucket, path);
        
        self.client
            .put_object()
            .bucket(&self.base_bucket)
            .key(key)
            .body(ByteStream::from(data))
            .send()
            .await
            .map_err(|e| EngineError::Storage(format!("S3 upload failed: {}", e)))?;

        Ok(())
    }

    async fn sign_url(&self, project_id: &str, bucket: &str, path: &str, expires_in: Duration) -> Result<String, EngineError> {
        let key = self.resolve_path(project_id, bucket, path);
        
        let presigning_config = PresigningConfig::expires_in(expires_in)
            .map_err(|e| EngineError::Storage(format!("Presigning config failed: {}", e)))?;

        let presigned_request = self.client
            .get_object()
            .bucket(&self.base_bucket)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| EngineError::Storage(format!("Presigning failed: {}", e)))?;

        Ok(presigned_request.uri().to_string())
    }
}
