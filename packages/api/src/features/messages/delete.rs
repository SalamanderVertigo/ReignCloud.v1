use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeleteMessageRequest {
    pub token: String,
    pub message_id: String,
}

#[post("/api/messages/delete")]
pub async fn delete_message(req: DeleteMessageRequest) -> Result<bool, ServerFnError> {
    use crate::auth::validate_token;
    use crate::db;

    let claims = validate_token(&req.token).map_err(|e| ServerFnError::new(format!("Unauthorized: {e}")))?;
    let user_id: uuid::Uuid = claims.sub.parse().map_err(|e: uuid::Error| ServerFnError::new(e.to_string()))?;
    let message_id: uuid::Uuid = req
        .message_id
        .parse()
        .map_err(|e: uuid::Error| ServerFnError::new(format!("Invalid message_id: {e}")))?;

    let result = sqlx::query("DELETE FROM messages WHERE id = $1 AND sender_id = $2")
        .bind(message_id)
        .bind(user_id)
        .execute(db::pool().await)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ServerFnError::new("Message not found or you are not the sender"));
    }

    Ok(true)
}
