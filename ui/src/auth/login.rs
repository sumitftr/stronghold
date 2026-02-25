use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut id = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(String::new);
    let mut show_username_step = use_signal(|| false);
    let mut oauth_email = use_signal(String::new);
    let mut new_username = use_signal(String::new);

    let handle_login = move |ev: Event<FormData>| async move {
        ev.prevent_default();
        is_loading.set(true);

        let id = id();
        let pass = password();

        // Validate input
        let (email_opt, username_opt) = match id.contains('@') {
            true => match shared::validation::is_email_valid(&id) {
                Ok(_) => (Some(id.clone()), None),
                Err(e) => {
                    error_message.set(e.to_string());
                    is_loading.set(false);
                    return;
                }
            },
            false => match shared::validation::is_username_valid(&id) {
                Ok(_) => (None, Some(id.clone())),
                Err(e) => {
                    error_message.set(e.to_string());
                    is_loading.set(false);
                    return;
                }
            },
        };

        // Send login request
        let url = format!("{}/api/login", crate::SERVICE_DOMAIN());

        match reqwest::Client::new()
            .post(&url)
            .json(&serde_json::json!({
                "email": email_opt,
                "username": username_opt,
                "password": pass,
            }))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    // Successfully logged in - redirect or update state
                    // You might want to save auth token, redirect to dashboard, etc.
                } else {
                    let error_text =
                        response.text().await.unwrap_or_else(|_| "Login failed".to_string());
                    error_message.set(error_text);
                }
            }
            Err(e) => {
                error_message.set(format!("Network error: {}", e));
            }
        }

        is_loading.set(false);
    };

    let handle_google_login = move |_| async move {
        is_loading.set(true);
        error_message.set(String::new());

        let url = format!("{}/api/oauth2/login", crate::SERVICE_DOMAIN());

        // Redirect to OAuth endpoint
        // In a real browser environment, you'd use window.location
        // For now, we'll simulate the flow
        match reqwest::Client::new().get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    // Check if we need to set username
                    // This is a simplified version - in reality, the backend would redirect
                    // and you'd check URL parameters or response body
                    if let Ok(body) = response.text().await {
                        if body.contains("set_username") {
                            // Extract email from response if needed
                            // For now, assuming it's in the response
                            oauth_email.set("user@example.com".to_string());
                            show_username_step.set(true);
                        }
                    }
                } else {
                    error_message.set("OAuth login failed".to_string());
                }
            }
            Err(e) => {
                error_message.set(format!("Network error: {}", e));
            }
        }

        is_loading.set(false);
    };

    let handle_username_complete = move |_| {
        show_username_step.set(false);
        // Redirect to dashboard or home
    };

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center px-4 bg-[var(--primary-color)]",

            div {
                class: "w-full max-w-md",

                // Show username step if needed
                if show_username_step() {
                    SetUsernameStep {
                        email: oauth_email,
                        username: new_username,
                        is_loading: is_loading,
                        on_complete: handle_username_complete,
                    }
                } else {
                    // Card container
                    div {
                        class: "rounded-lg border p-8 shadow-sm bg-[var(--primary-color-1)] border-[var(--primary-color-6)]",

                        // Header
                        div {
                            class: "flex flex-col space-y-2 text-center mb-6",
                            h1 {
                                class: "text-2xl font-semibold tracking-tight text-[var(--secondary-color-1)]",
                                "Welcome back"
                            }
                            p {
                                class: "text-sm text-[var(--secondary-color-5)]",
                                "Enter your credentials to sign in to your account"
                            }
                        }

                        // Error message
                        if !error_message().is_empty() {
                            div {
                                class: "mb-4 p-3 rounded-md text-sm bg-[#fee] text-[#c33]",
                                style: "border: 1px solid #fcc;",
                                {error_message()}
                            }
                        }

                        // Form
                        form {
                            class: "space-y-4",
                            onsubmit: handle_login,

                            // Email/Username field
                            div {
                                class: "space-y-2",
                                label {
                                    class: "text-sm font-medium text-[var(--secondary-color-2)]",
                                    r#for: "email",
                                    "Email or Username"
                                }
                                input {
                                    class: "flex h-10 w-full rounded-md border px-3 py-2 text-sm transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 bg-[var(--primary-color-3)] text-[var(--secondary-color-1)]",
                                    style: "border-color: var(--primary-color-6); focus:ring-color: var(--focused-border-color);",
                                    r#type: "text",
                                    id: "email",
                                    placeholder: "name@example.com",
                                    value: "{id}",
                                    oninput: move |e| id.set(e.value()),
                                    required: true,
                                }
                            }

                            // Password field
                            div {
                                class: "space-y-2",
                                div {
                                    class: "flex items-center justify-between",
                                    label {
                                        class: "text-sm font-medium text-[var(--secondary-color-2)]",
                                        r#for: "password",
                                        "Password"
                                    }
                                    a {
                                        href: "/forgot-password",
                                        class: "text-sm font-medium hover:underline text-[var(--focused-border-color)]",
                                        "Forgot password?"
                                    }
                                }
                                input {
                                    class: "flex h-10 w-full rounded-md border px-3 py-2 text-sm transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 bg-[var(--primary-color-3)] text-[var(--secondary-color-1)]",
                                    style: "border-color: var(--primary-color-6); focus:ring-color: var(--focused-border-color);",
                                    r#type: "password",
                                    id: "password",
                                    placeholder: "••••••••",
                                    value: "{password}",
                                    oninput: move |e| password.set(e.value()),
                                    required: true,
                                }
                            }

                            // Submit button
                            button {
                                class: "w-full h-10 rounded-md text-sm font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed bg-[var(--secondary-color-1)] text-[var(--primary-color)]",
                                r#type: "submit",
                                disabled: is_loading(),
                                if is_loading() {
                                    "Signing in..."
                                } else {
                                    "Login"
                                }
                            }
                        }

                        // Divider
                        div {
                            class: "relative my-6",
                            div {
                                class: "absolute inset-0 flex items-center",
                                div {
                                    class: "w-full border-t",
                                    style: "border-color: var(--primary-color-6);",
                                }
                            }
                            div {
                                class: "relative flex justify-center text-xs uppercase",
                                span {
                                    class: "px-2 text-xs bg-[var(--primary-color-1)] text-[var(--secondary-color-5)]",
                                    "Or continue with"
                                }
                            }
                        }

                        // Google login button
                        button {
                            class: "w-full h-10 rounded-md border text-sm font-medium transition-colors flex items-center justify-center gap-2 bg-[var(--primary-color-3)] text-[var(--secondary-color-1)]",
                            style: "border-color: var(--primary-color-6);",
                            onclick: handle_google_login,
                            disabled: is_loading(),
                            // Google icon
                            svg {
                                class: "h-5 w-5",
                                view_box: "0 0 24 24",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 48 48",
                                    path {
                                        fill: "#FFC107",
                                        d: "M43.611,20.083H42V20H24v8h11.303c-1.649,4.657-6.08,8-11.303,8c-6.627,0-12-5.373-12-12c0-6.627,5.373-12,12-12c3.059,0,5.842,1.154,7.961,3.039l5.657-5.657C34.046,6.053,29.268,4,24,4C12.955,4,4,12.955,4,24c0,11.045,8.955,20,20,20c11.045,0,20-8.955,20-20C44,22.659,43.862,21.35,43.611,20.083z"
                                    }
                                    path {
                                        fill: "#FF3D00",
                                        d: "M6.306,14.691l6.571,4.819C14.655,15.108,18.961,12,24,12c3.059,0,5.842,1.154,7.961,3.039l5.657-5.657C34.046,6.053,29.268,4,24,4C16.318,4,9.656,8.337,6.306,14.691z"
                                    }
                                    path {
                                        fill: "#4CAF50",
                                        d: "M24,44c5.166,0,9.86-1.977,13.409-5.192l-6.19-5.238C29.211,35.091,26.715,36,24,36c-5.202,0-9.619-3.317-11.283-7.946l-6.522,5.025C9.505,39.556,16.227,44,24,44z"
                                    }
                                    path {
                                        fill: "#1976D2",
                                        d: "M43.611,20.083H42V20H24v8h11.303c-0.792,2.237-2.231,4.166-4.087,5.571c0.001-0.001,0.002-0.001,0.003-0.002l6.19,5.238C36.971,39.205,44,34,44,24C44,22.659,43.862,21.35,43.611,20.083z"
                                    }
                                }
                            }
                            "Continue with Google"
                        }

                        // Register link
                        div {
                            class: "mt-6 text-center text-sm text-[var(--secondary-color-5)]",
                            "Don't have an account? "
                            Link {
                                to: Route::Register {},
                                class: "font-medium hover:underline text-[var(--focused-border-color)]",
                                "Register"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SetUsernameStep(
    email: Signal<String>,
    username: Signal<String>,
    is_loading: Signal<bool>,
    on_complete: EventHandler<()>,
) -> Element {
    let mut error_message = use_signal(String::new);

    let handle_submit = move |ev: Event<FormData>| async move {
        ev.prevent_default();
        is_loading.set(true);
        error_message.set(String::new());

        let email_val = email();
        let username_val = username();

        // Validate username
        match shared::validation::is_username_valid(&username_val) {
            Ok(_) => {
                let url = format!("{}/api/register/set_username", crate::SERVICE_DOMAIN());

                match reqwest::Client::new()
                    .post(&url)
                    .json(&serde_json::json!({
                        "email": email_val,
                        "username": username_val,
                    }))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            on_complete.call(());
                        } else {
                            let error_text = response
                                .text()
                                .await
                                .unwrap_or_else(|_| "Failed to set username".to_string());
                            error_message.set(error_text);
                        }
                    }
                    Err(e) => {
                        error_message.set(format!("Network error: {}", e));
                    }
                }
            }
            Err(e) => {
                error_message.set(e.to_string());
            }
        }

        is_loading.set(false);
    };

    rsx! {
        div {
            class: "rounded-lg border p-8 shadow-sm",
            style: "background-color: var(--primary-color-1); border-color: var(--primary-color-6);",

            // Header
            div {
                class: "flex flex-col space-y-2 text-center mb-6",
                h1 {
                    class: "text-2xl font-semibold tracking-tight",
                    style: "color: var(--secondary-color-1);",
                    "Choose a username"
                }
                p {
                    class: "text-sm",
                    style: "color: var(--secondary-color-5);",
                    "Complete your account setup by choosing a unique username"
                }
            }

            // Error message
            if !error_message().is_empty() {
                div {
                    class: "mb-4 p-3 rounded-md text-sm",
                    style: "background-color: #fee; border: 1px solid #fcc; color: #c33;",
                    {error_message()}
                }
            }

            // Form
            form {
                class: "space-y-4",
                onsubmit: handle_submit,

                // Username field
                div {
                    class: "space-y-2",
                    label {
                        class: "text-sm font-medium",
                        style: "color: var(--secondary-color-2);",
                        r#for: "username",
                        "Username"
                    }
                    input {
                        class: "flex h-10 w-full rounded-md border px-3 py-2 text-sm transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1); focus:ring-color: var(--focused-border-color);",
                        r#type: "text",
                        id: "username",
                        placeholder: "johndoe",
                        value: "{username}",
                        oninput: move |e| username.set(e.value()),
                        required: true,
                    }
                }

                // Submit button
                button {
                    class: "w-full h-10 rounded-md text-sm font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                    style: "background-color: var(--secondary-color-1); color: var(--primary-color);",
                    r#type: "submit",
                    disabled: is_loading(),
                    if is_loading() {
                        "Setting username..."
                    } else {
                        "Continue"
                    }
                }
            }
        }
    }
}
