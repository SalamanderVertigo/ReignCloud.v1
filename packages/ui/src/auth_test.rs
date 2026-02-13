use dioxus::prelude::*;

/// Temporary smoke-test component for auth + messages.
#[component]
pub fn AuthTest() -> Element {
    // Auth state
    let mut email = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut token = use_signal(|| String::new());
    let mut result_text = use_signal(|| String::new());

    // Message state
    let mut recipient_id = use_signal(|| String::new());
    let mut message_content = use_signal(|| String::new());
    let mut message_id = use_signal(|| String::new());

    let handle_register = move |_| async move {
        let req = api::features::users::register::RegisterRequest {
            email: email(),
            username: username(),
            password: password(),
        };
        match api::register(req).await {
            Ok(tokens) => {
                token.set(tokens.access_token.clone());
                result_text.set(format!(
                    "Registration successful!\nToken stored for message testing.\n\nAccess token:\n{}",
                    tokens.access_token
                ));
            }
            Err(e) => result_text.set(format!("Registration failed: {e}")),
        }
    };

    let handle_login = move |_| async move {
        let req = api::features::users::login::LoginRequest {
            email: email(),
            password: password(),
        };
        match api::login(req).await {
            Ok(tokens) => {
                token.set(tokens.access_token.clone());
                result_text.set(format!(
                    "Login successful!\nToken stored for message testing.\n\nAccess token:\n{}",
                    tokens.access_token
                ));
            }
            Err(e) => result_text.set(format!("Login failed: {e}")),
        }
    };

    let handle_send = move |_| async move {
        let req = api::features::messages::create::CreateMessageRequest {
            token: token(),
            recipient_id: recipient_id(),
            content: message_content(),
        };
        match api::create_message(req).await {
            Ok(msg) => {
                message_id.set(msg.id.clone());
                result_text.set(format!(
                    "Message sent!\nid: {}\nsender: {}\nrecipient: {}\ncontent: {}\ncreated_at: {}",
                    msg.id, msg.sender_id, msg.recipient_id, msg.content, msg.created_at
                ));
            }
            Err(e) => result_text.set(format!("Send failed: {e}")),
        }
    };

    let handle_list = move |_| async move {
        let req = api::features::messages::list::ListMessagesRequest {
            token: token(),
            other_user_id: recipient_id(),
        };
        match api::list_messages(req).await {
            Ok(msgs) => {
                let mut out = format!("Found {} messages:\n\n", msgs.len());
                for m in &msgs {
                    out.push_str(&format!(
                        "[{}] {} -> {}: {}\n",
                        m.created_at, m.sender_id, m.recipient_id, m.content
                    ));
                }
                result_text.set(out);
            }
            Err(e) => result_text.set(format!("List failed: {e}")),
        }
    };

    let handle_update = move |_| async move {
        let req = api::features::messages::update::UpdateMessageRequest {
            token: token(),
            message_id: message_id(),
            content: message_content(),
        };
        match api::update_message(req).await {
            Ok(msg) => {
                result_text.set(format!(
                    "Message updated!\nid: {}\ncontent: {}",
                    msg.id, msg.content
                ));
            }
            Err(e) => result_text.set(format!("Update failed: {e}")),
        }
    };

    let handle_delete = move |_| async move {
        let req = api::features::messages::delete::DeleteMessageRequest {
            token: token(),
            message_id: message_id(),
        };
        match api::delete_message(req).await {
            Ok(_) => result_text.set("Message deleted!".to_string()),
            Err(e) => result_text.set(format!("Delete failed: {e}")),
        }
    };

    rsx! {
        div {
            style: "max-width: 500px; margin: 2rem auto; padding: 1rem; border: 1px solid #ccc; border-radius: 8px;",

            h3 { "Auth Smoke Test" }
            input {
                style: "display: block; width: 100%; margin: 0.5rem 0; padding: 0.5rem;",
                placeholder: "Email",
                value: "{email}",
                oninput: move |e| email.set(e.value()),
            }
            input {
                style: "display: block; width: 100%; margin: 0.5rem 0; padding: 0.5rem;",
                placeholder: "Username",
                value: "{username}",
                oninput: move |e| username.set(e.value()),
            }
            input {
                style: "display: block; width: 100%; margin: 0.5rem 0; padding: 0.5rem;",
                r#type: "password",
                placeholder: "Password (min 8 chars)",
                value: "{password}",
                oninput: move |e| password.set(e.value()),
            }
            div {
                style: "display: flex; gap: 0.5rem; margin-top: 0.5rem;",
                button { onclick: handle_register, "Register" }
                button { onclick: handle_login, "Login" }
            }

            hr { style: "margin: 1rem 0;" }

            h3 { "Messages Smoke Test" }
            if token().is_empty() {
                p { style: "color: #888;", "Login or register first to get a token." }
            }
            input {
                style: "display: block; width: 100%; margin: 0.5rem 0; padding: 0.5rem;",
                placeholder: "Recipient User ID (UUID)",
                value: "{recipient_id}",
                oninput: move |e| recipient_id.set(e.value()),
            }
            input {
                style: "display: block; width: 100%; margin: 0.5rem 0; padding: 0.5rem;",
                placeholder: "Message content",
                value: "{message_content}",
                oninput: move |e| message_content.set(e.value()),
            }
            input {
                style: "display: block; width: 100%; margin: 0.5rem 0; padding: 0.5rem;",
                placeholder: "Message ID (for update/delete, auto-filled on send)",
                value: "{message_id}",
                oninput: move |e| message_id.set(e.value()),
            }
            div {
                style: "display: flex; gap: 0.5rem; flex-wrap: wrap; margin-top: 0.5rem;",
                button { onclick: handle_send, "Send" }
                button { onclick: handle_list, "List" }
                button { onclick: handle_update, "Update" }
                button { onclick: handle_delete, "Delete" }
            }

            if !result_text().is_empty() {
                pre {
                    style: "margin-top: 1rem; padding: 0.5rem; background: #f0f0f0; border-radius: 4px; white-space: pre-wrap; word-break: break-all; font-size: 0.8rem;",
                    "{result_text}"
                }
            }
        }
    }
}
