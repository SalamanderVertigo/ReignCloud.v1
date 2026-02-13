use dioxus::prelude::*;

use api::features::messages::create::MessageResponse;

/// Hook that manages a WebSocket connection for real-time messages.
/// Returns a signal containing incoming messages.
///
/// Pass the JWT access token to connect. When the token changes
/// (e.g. on login), the connection is re-established.
pub fn use_websocket(token: Signal<String>) -> Signal<Vec<MessageResponse>> {
    let mut messages: Signal<Vec<MessageResponse>> = use_signal(Vec::new);

    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            let tok = token();
            if tok.is_empty() {
                return;
            }

            spawn(async move {
                use gloo_net::websocket::{futures::WebSocket, Message};
                use futures_util::StreamExt;

                // Build ws:// or wss:// URL relative to current host
                let location = web_sys::window().unwrap().location();
                let protocol = location.protocol().unwrap_or_default();
                let host = location.host().unwrap_or_default();
                let ws_protocol = if protocol == "https:" { "wss:" } else { "ws:" };
                let url = format!("{ws_protocol}//{host}/ws?token={tok}");

                let ws = match WebSocket::open(&url) {
                    Ok(ws) => ws,
                    Err(e) => {
                        web_sys::console::log_1(
                            &format!("WebSocket connect failed: {e:?}").into(),
                        );
                        return;
                    }
                };

                let (_write, mut read) = ws.split();

                web_sys::console::log_1(&"WebSocket connected".into());

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            if let Ok(msg_response) =
                                serde_json::from_str::<MessageResponse>(&text)
                            {
                                messages.write().push(msg_response);
                            }
                        }
                        Ok(Message::Bytes(_)) => {}
                        Err(e) => {
                            web_sys::console::log_1(
                                &format!("WebSocket error: {e:?}").into(),
                            );
                            break;
                        }
                    }
                }

                web_sys::console::log_1(&"WebSocket disconnected".into());
            });
        });
    }

    messages
}
