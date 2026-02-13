use dioxus::prelude::*;

/// Temporary smoke-test component for registration and login.
#[component]
pub fn AuthTest() -> Element {
    let mut email = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut result_text = use_signal(|| String::new());

    let handle_register = move |_| async move {
        let req = api::features::users::register::RegisterRequest {
            email: email(),
            username: username(),
            password: password(),
        };
        match api::register(req).await {
            Ok(tokens) => {
                result_text.set(format!(
                    "Registration successful!\n\nAccess token:\n{}\n\nRefresh token:\n{}",
                    tokens.access_token, tokens.refresh_token
                ));
            }
            Err(e) => {
                result_text.set(format!("Registration failed: {e}"));
            }
        }
    };

    let handle_login = move |_| async move {
        let req = api::features::users::login::LoginRequest {
            email: email(),
            password: password(),
        };
        match api::login(req).await {
            Ok(tokens) => {
                result_text.set(format!(
                    "Login successful!\n\nAccess token:\n{}\n\nRefresh token:\n{}",
                    tokens.access_token, tokens.refresh_token
                ));
            }
            Err(e) => {
                result_text.set(format!("Login failed: {e}"));
            }
        }
    };

    rsx! {
        div {
            style: "max-width: 400px; margin: 2rem auto; padding: 1rem; border: 1px solid #ccc; border-radius: 8px;",
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
                style: "display: flex; gap: 0.5rem; margin-top: 1rem;",
                button {
                    onclick: handle_register,
                    "Register"
                }
                button {
                    onclick: handle_login,
                    "Login"
                }
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
