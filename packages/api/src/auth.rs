#[cfg(not(target_arch = "wasm32"))]
use chrono::{Duration, Utc};
#[cfg(not(target_arch = "wasm32"))]
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use uuid::Uuid;

/// JWT claims payload.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub exp: usize, // expiry timestamp
    pub iat: usize, // issued at
}

/// Token pair returned on login/register.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env")
}

/// Create an access token (short-lived, 15 min).
#[cfg(not(target_arch = "wasm32"))]
pub fn create_access_token(user_id: Uuid, email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::minutes(15)).timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

/// Create a refresh token (long-lived, 7 days).
#[cfg(not(target_arch = "wasm32"))]
pub fn create_refresh_token(user_id: Uuid, email: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::days(7)).timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

/// Validate a token and return the claims.
#[cfg(not(target_arch = "wasm32"))]
pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
