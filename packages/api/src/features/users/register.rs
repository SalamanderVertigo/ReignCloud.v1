use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::auth::TokenPair;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[post("/api/users/register")]
pub async fn register(req: RegisterRequest) -> Result<TokenPair, ServerFnError> {
    use crate::auth::{create_access_token, create_refresh_token};
    use crate::db;

    // Validate input
    if req.email.is_empty() || req.username.is_empty() || req.password.is_empty() {
        return Err(ServerFnError::new("All fields are required"));
    }

    if req.password.len() < 8 {
        return Err(ServerFnError::new("Password must be at least 8 characters"));
    }

    // Hash password
    let password_hash =
        bcrypt::hash(&req.password, bcrypt::DEFAULT_COST).map_err(|e| ServerFnError::new(e.to_string()))?;

    // Insert user
    let user = sqlx::query_as::<_, (uuid::Uuid,)>(
        "INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(&req.email)
    .bind(&req.username)
    .bind(&password_hash)
    .fetch_one(db::pool().await)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            ServerFnError::new("A user with that email or username already exists")
        }
        _ => ServerFnError::new(e.to_string()),
    })?;

    let user_id = user.0;

    // Generate tokens
    let access_token =
        create_access_token(user_id, &req.email).map_err(|e| ServerFnError::new(e.to_string()))?;
    let refresh_token =
        create_refresh_token(user_id, &req.email).map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(TokenPair {
        access_token,
        refresh_token,
    })
}
