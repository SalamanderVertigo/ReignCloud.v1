use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateMessageRequest {
    pub token: String,
    pub recipient_id: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageResponse {
    pub id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub content: String,
    pub created_at: String,
}

#[post("/api/messages/create")]
pub async fn create_message(req: CreateMessageRequest) -> Result<MessageResponse, ServerFnError> {
    use crate::auth::validate_token;
    use crate::db;

    let claims = validate_token(&req.token).map_err(|e| ServerFnError::new(format!("Unauthorized: {e}")))?;
    let sender_id: uuid::Uuid = claims.sub.parse().map_err(|e: uuid::Error| ServerFnError::new(e.to_string()))?;
    let recipient_id: uuid::Uuid = req
        .recipient_id
        .parse()
        .map_err(|e: uuid::Error| ServerFnError::new(format!("Invalid recipient_id: {e}")))?;

    if req.content.is_empty() {
        return Err(ServerFnError::new("Message content cannot be empty"));
    }

    let row = sqlx::query_as::<_, (uuid::Uuid, chrono::DateTime<chrono::Utc>)>(
        "INSERT INTO messages (sender_id, recipient_id, content) VALUES ($1, $2, $3) RETURNING id, created_at",
    )
    .bind(sender_id)
    .bind(recipient_id)
    .bind(&req.content)
    .fetch_one(db::pool().await)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(MessageResponse {
        id: row.0.to_string(),
        sender_id: sender_id.to_string(),
        recipient_id: recipient_id.to_string(),
        content: req.content,
        created_at: row.1.to_rfc3339(),
    })
}
