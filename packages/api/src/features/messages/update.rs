use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::create::MessageResponse;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateMessageRequest {
    pub token: String,
    pub message_id: String,
    pub content: String,
}

#[post("/api/messages/update")]
pub async fn update_message(req: UpdateMessageRequest) -> Result<MessageResponse, ServerFnError> {
    use crate::auth::validate_token;
    use crate::db;

    let claims = validate_token(&req.token).map_err(|e| ServerFnError::new(format!("Unauthorized: {e}")))?;
    let user_id: uuid::Uuid = claims.sub.parse().map_err(|e: uuid::Error| ServerFnError::new(e.to_string()))?;
    let message_id: uuid::Uuid = req
        .message_id
        .parse()
        .map_err(|e: uuid::Error| ServerFnError::new(format!("Invalid message_id: {e}")))?;

    if req.content.is_empty() {
        return Err(ServerFnError::new("Message content cannot be empty"));
    }

    let row = sqlx::query_as::<_, (uuid::Uuid, uuid::Uuid, uuid::Uuid, String, chrono::DateTime<chrono::Utc>)>(
        "UPDATE messages SET content = $1, updated_at = NOW()
         WHERE id = $2 AND sender_id = $3
         RETURNING id, sender_id, recipient_id, content, created_at",
    )
    .bind(&req.content)
    .bind(message_id)
    .bind(user_id)
    .fetch_optional(db::pool().await)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let r = row.ok_or_else(|| ServerFnError::new("Message not found or you are not the sender"))?;

    Ok(MessageResponse {
        id: r.0.to_string(),
        sender_id: r.1.to_string(),
        recipient_id: r.2.to_string(),
        content: r.3,
        created_at: r.4.to_rfc3339(),
    })
}
