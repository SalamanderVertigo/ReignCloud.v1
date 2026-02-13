use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::auth::TokenPair;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/api/users/login")]
pub async fn login(req: LoginRequest) -> Result<TokenPair, ServerFnError> {
    use crate::auth::{create_access_token, create_refresh_token};
    use crate::db;

    if req.email.is_empty() || req.password.is_empty() {
        return Err(ServerFnError::new("Email and password are required"));
    }

    // Look up user by email
    let row = sqlx::query_as::<_, (uuid::Uuid, String)>(
        "SELECT id, password_hash FROM users WHERE email = $1",
    )
    .bind(&req.email)
    .fetch_optional(db::pool().await)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let (user_id, password_hash) = row.ok_or_else(|| ServerFnError::new("Invalid email or password"))?;

    // Verify password
    let valid =
        bcrypt::verify(&req.password, &password_hash).map_err(|e| ServerFnError::new(e.to_string()))?;

    if !valid {
        return Err(ServerFnError::new("Invalid email or password"));
    }

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
