use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use super::create::MessageResponse;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListMessagesRequest {
    pub token: String,
    pub other_user_id: String,
}

#[post("/api/messages/list")]
pub async fn list_messages(req: ListMessagesRequest) -> Result<Vec<MessageResponse>, ServerFnError> {
    use crate::auth::validate_token;
    use crate::db;

    let claims = validate_token(&req.token).map_err(|e| ServerFnError::new(format!("Unauthorized: {e}")))?;
    let user_id: uuid::Uuid = claims.sub.parse().map_err(|e: uuid::Error| ServerFnError::new(e.to_string()))?;
    let other_id: uuid::Uuid = req
        .other_user_id
        .parse()
        .map_err(|e: uuid::Error| ServerFnError::new(format!("Invalid other_user_id: {e}")))?;

    let rows = sqlx::query_as::<_, (uuid::Uuid, uuid::Uuid, uuid::Uuid, String, chrono::DateTime<chrono::Utc>)>(
        "SELECT id, sender_id, recipient_id, content, created_at FROM messages
         WHERE (sender_id = $1 AND recipient_id = $2) OR (sender_id = $2 AND recipient_id = $1)
         ORDER BY created_at ASC",
    )
    .bind(user_id)
    .bind(other_id)
    .fetch_all(db::pool().await)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let messages = rows
        .into_iter()
        .map(|r| MessageResponse {
            id: r.0.to_string(),
            sender_id: r.1.to_string(),
            recipient_id: r.2.to_string(),
            content: r.3,
            created_at: r.4.to_rfc3339(),
        })
        .collect();

    Ok(messages)
}
