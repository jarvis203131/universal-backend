use axum::{
    extract::State,
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use engine::EngineError;
use auth::{TokenManager, AuthHasher};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub token_manager: Arc<TokenManager>,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub project_id: Uuid,
    pub email: String,
    pub password: String,
    pub full_name: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub project_id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: Uuid,
}

#[derive(Serialize)]
pub struct UserMeResponse {
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub email: String,
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, EngineError> {
    let password_hash = AuthHasher::hash(&payload.password)?;

    let user = sqlx::query_as::<_, UserRecord>(
        "INSERT INTO users (project_id, email, password_hash, full_name) VALUES ($1, $2, $3, $4) RETURNING id, project_id, email"
    )
    .bind(payload.project_id)
    .bind(payload.email)
    .bind(password_hash)
    .bind(payload.full_name)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        if let Some(db_err) = e.as_database_error() {
            if db_err.is_unique_violation() {
                return EngineError::UserExists;
            }
        }
        EngineError::Database(e)
    })?;

    let access_token = state.token_manager.generate_access_token(user.id, user.project_id)?;
    let refresh_token = state.token_manager.generate_refresh_token(user.id, user.project_id)?;

    Ok((StatusCode::CREATED, Json(AuthResponse {
        access_token,
        refresh_token,
        user_id: user.id,
    })))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, EngineError> {
    let user = sqlx::query_as::<_, UserLoginRecord>(
        "SELECT id, password_hash, project_id FROM users WHERE project_id = $1 AND email = $2"
    )
    .bind(payload.project_id)
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| EngineError::Authentication("Invalid credentials".to_string()))?;

    if !AuthHasher::verify(&payload.password, &user.password_hash)? {
        return Err(EngineError::WrongPassword);
    }

    let access_token = state.token_manager.generate_access_token(user.id, user.project_id)?;
    let refresh_token = state.token_manager.generate_refresh_token(user.id, user.project_id)?;

    Ok((StatusCode::OK, Json(AuthResponse {
        access_token,
        refresh_token,
        user_id: user.id,
    })))
}

async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshRequest>,
) -> Result<impl IntoResponse, EngineError> {
    let claims = state.token_manager.verify_token(&payload.refresh_token, true)?;
    
    let access_token = state.token_manager.generate_access_token(claims.sub, claims.pid)?;
    let refresh_token = state.token_manager.generate_refresh_token(claims.sub, claims.pid)?;

    Ok((StatusCode::OK, Json(AuthResponse {
        access_token,
        refresh_token,
        user_id: claims.sub,
    })))
}

async fn me(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, EngineError> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| EngineError::Authentication("Missing authorization header".to_string()))?;

    let claims = state.token_manager.verify_token(auth_header, false)?;

    let user_email: String = sqlx::query_scalar(
        "SELECT email FROM users WHERE id = $1 AND project_id = $2"
    )
    .bind(claims.sub)
    .bind(claims.pid)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| EngineError::Authentication("User not found".to_string()))?;

    Ok(Json(UserMeResponse {
        user_id: claims.sub,
        project_id: claims.pid,
        email: user_email,
    }))
}

async fn health_check() -> &'static str {
    "OK"
}

#[derive(sqlx::FromRow)]
struct UserRecord {
    id: Uuid,
    project_id: Uuid,
    email: String,
}

#[derive(sqlx::FromRow)]
struct UserLoginRecord {
    id: Uuid,
    password_hash: String,
    project_id: Uuid,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/universal_backend".to_string());
    let pool = PgPool::connect(&db_url).await.expect("Failed to connect to Postgres");

    let token_manager = Arc::new(TokenManager::new(
        "access_secret_key_123".to_string(),
        "refresh_secret_key_456".to_string(),
    ));

    let state = Arc::new(AppState {
        db: pool,
        token_manager,
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh))
        .route("/auth/me", get(me))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Gateway listening on :8080");
    axum::serve(listener, app).await.unwrap();
}
