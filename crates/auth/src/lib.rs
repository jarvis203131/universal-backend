use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordVerifier, SaltString},
    Argon2, PasswordHasher as Argon2PasswordHasher,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, Duration};
use engine::EngineError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,       // user_id
    pub pid: Uuid,       // project_id
    pub exp: usize,
}

pub struct AuthHasher;

impl AuthHasher {
    pub fn hash(password: &str) -> Result<String, EngineError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| EngineError::Internal(anyhow::anyhow!("Hashing failed: {}", e)))
    }

    pub fn verify(password: &str, hash: &str) -> Result<bool, EngineError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| EngineError::Internal(anyhow::anyhow!("Invalid hash format: {}", e)))?;
        
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

pub struct TokenManager {
    secret: Vec<u8>,
    refresh_secret: Vec<u8>,
}

impl TokenManager {
    pub fn new(secret: String, refresh_secret: String) -> Self {
        Self {
            secret: secret.into_bytes(),
            refresh_secret: refresh_secret.into_bytes(),
        }
    }

    pub fn generate_access_token(&self, user_id: Uuid, project_id: Uuid) -> Result<String, EngineError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(15))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            pid: project_id,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .map_err(|e| EngineError::Internal(anyhow::anyhow!("Token encoding failed: {}", e)))
    }

    pub fn generate_refresh_token(&self, user_id: Uuid, project_id: Uuid) -> Result<String, EngineError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(7))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            pid: project_id,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.refresh_secret),
        )
        .map_err(|e| EngineError::Internal(anyhow::anyhow!("Refresh token encoding failed: {}", e)))
    }

    pub fn verify_token(&self, token: &str, is_refresh: bool) -> Result<Claims, EngineError> {
        let secret = if is_refresh { &self.refresh_secret } else { &self.secret };
        
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => EngineError::ExpiredToken,
            _ => EngineError::InvalidToken,
        })
    }
}
