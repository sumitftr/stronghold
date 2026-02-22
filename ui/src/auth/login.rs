use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut is_loading = use_signal(|| false);

    let handle_login = move |ev: Event<FormData>| async move {
        ev.prevent_default();
        is_loading.set(true);
        // TODO: Add validation logic here

        // TODO: Send login request here

        is_loading.set(false);
    };

    let handle_google_login = move |_| async move {
        // TODO: Handle Google OAuth login
    };

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center px-4",
            style: "background-color: var(--primary-color);",

            div {
                class: "w-full max-w-md",

                // Card container
                div {
                    class: "rounded-lg border p-8 shadow-sm",
                    style: "background-color: var(--primary-color-1); border-color: var(--primary-color-6);",

                    // Header
                    div {
                        class: "flex flex-col space-y-2 text-center mb-6",
                        h1 {
                            class: "text-2xl font-semibold tracking-tight",
                            style: "color: var(--secondary-color-1);",
                            "Welcome back"
                        }
                        p {
                            class: "text-sm",
                            style: "color: var(--secondary-color-5);",
                            "Enter your credentials to sign in to your account"
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
                                class: "text-sm font-medium",
                                style: "color: var(--secondary-color-2);",
                                r#for: "email",
                                "Email or Username"
                            }
                            input {
                                class: "flex h-10 w-full rounded-md border px-3 py-2 text-sm transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
                                style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1); focus:ring-color: var(--focused-border-color);",
                                r#type: "text",
                                id: "email",
                                placeholder: "name@example.com",
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                                required: true,
                            }
                        }

                        // Password field
                        div {
                            class: "space-y-2",
                            div {
                                class: "flex items-center justify-between",
                                label {
                                    class: "text-sm font-medium",
                                    style: "color: var(--secondary-color-2);",
                                    r#for: "password",
                                    "Password"
                                }
                                a {
                                    href: "/forgot-password",
                                    class: "text-sm font-medium hover:underline",
                                    style: "color: var(--focused-border-color);",
                                    "Forgot password?"
                                }
                            }
                            input {
                                class: "flex h-10 w-full rounded-md border px-3 py-2 text-sm transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
                                style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1); focus:ring-color: var(--focused-border-color);",
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
                            class: "w-full h-10 rounded-md text-sm font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                            style: "background-color: var(--secondary-color-1); color: var(--primary-color);",
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
                                class: "px-2 text-xs",
                                style: "background-color: var(--primary-color-1); color: var(--secondary-color-5);",
                                "Or continue with"
                            }
                        }
                    }

                    // Google login button
                    button {
                        class: "w-full h-10 rounded-md border text-sm font-medium transition-colors flex items-center justify-center gap-2",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1);",
                        onclick: handle_google_login,
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
                        class: "mt-6 text-center text-sm",
                        style: "color: var(--secondary-color-5);",
                        "Don't have an account? "
                        Link {
                            to: Route::Register {},
                            class: "font-medium hover:underline",
                            style: "color: var(--focused-border-color);",
                            "Register"
                        }
                    }
                }
            }
        }
    }
}
