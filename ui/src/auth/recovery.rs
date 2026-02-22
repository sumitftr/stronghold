use dioxus::prelude::*;

#[component]
pub fn ForgotPassword() -> Element {
    let mut email = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut email_sent = use_signal(|| false);

    let handle_submit = move |ev: Event<FormData>| async move {
        ev.prevent_default();
        is_loading.set(true);

        // TODO: Send ForgotPasswordRequest

        is_loading.set(false);
        email_sent.set(true);
    };

    let handle_resend = move |_| async move {
        // TODO: Resend ForgotPasswordRequest
    };

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center px-4",
            style: "background-color: var(--primary-color);",

            div {
                class: "w-full max-w-md",

                div {
                    class: "rounded-lg border p-8 shadow-sm",
                    style: "background-color: var(--primary-color-1); border-color: var(--primary-color-6);",

                    if !email_sent() {
                        // Enter email form
                        div {
                            // Header
                            div {
                                class: "flex flex-col space-y-2 mb-6",
                                h1 {
                                    class: "text-2xl font-semibold tracking-tight",
                                    style: "color: var(--secondary-color-1);",
                                    "Reset your password"
                                }
                                p {
                                    class: "text-base",
                                    style: "color: var(--secondary-color-5);",
                                    "Enter your email address and we'll send you a link to reset your password"
                                }
                            }

                            // Form
                            form {
                                class: "space-y-5",
                                onsubmit: handle_submit,

                                // Email field
                                div {
                                    class: "space-y-2",
                                    label {
                                        class: "text-base font-medium",
                                        style: "color: var(--secondary-color-2);",
                                        r#for: "email",
                                        "Email Address"
                                    }
                                    input {
                                        class: "flex h-11 w-full rounded-md border px-4 py-2 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
                                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1); focus:ring-color: var(--focused-border-color);",
                                        r#type: "email",
                                        id: "email",
                                        placeholder: "name@example.com",
                                        value: "{email}",
                                        oninput: move |e| email.set(e.value()),
                                        required: true,
                                        autofocus: true,
                                    }
                                }

                                // Action buttons
                                div {
                                    class: "flex gap-3 pt-2",
                                    button {
                                        class: "flex-1 h-11 rounded-md border text-base font-medium transition-colors",
                                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1);",
                                        r#type: "button",
                                        onclick: move |_| {
                                            router().go_back();
                                        },
                                        "Cancel"
                                    }
                                    button {
                                        class: "flex-1 h-11 rounded-md text-base font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                                        style: "background-color: var(--secondary-color-1); color: var(--primary-color);",
                                        r#type: "submit",
                                        disabled: is_loading(),
                                        if is_loading() {
                                            "Sending..."
                                        } else {
                                            "Send reset link"
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Email sent confirmation
                        div {
                            // Success icon
                            div {
                                class: "flex justify-center mb-6",
                                div {
                                    class: "w-16 h-16 rounded-full flex items-center justify-center",
                                    style: "background-color: var(--primary-success-color);",
                                    svg {
                                        class: "w-8 h-8",
                                        style: "color: var(--secondary-success-color);",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            d: "M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                                        }
                                    }
                                }
                            }

                            // Header
                            div {
                                class: "flex flex-col space-y-2 text-center mb-6",
                                h2 {
                                    class: "text-2xl font-semibold tracking-tight",
                                    style: "color: var(--secondary-color-1);",
                                    "Check your email"
                                }
                                p {
                                    class: "text-base",
                                    style: "color: var(--secondary-color-5);",
                                    "We've sent a password reset link to"
                                }
                                p {
                                    class: "text-base font-medium",
                                    style: "color: var(--secondary-color-1);",
                                    "{email}"
                                }
                            }

                            // Instructions
                            div {
                                class: "rounded-lg border p-4 mb-6",
                                style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6);",
                                ul {
                                    class: "space-y-2 text-sm",
                                    style: "color: var(--secondary-color-5);",
                                    li { "• Click the link in the email to reset your password" }
                                    li { "• The link will expire in 1 hour" }
                                    li { "• Check your spam folder if you don't see it" }
                                }
                            }

                            // Resend and back buttons
                            div {
                                class: "space-y-3",
                                div {
                                    class: "text-center",
                                    span {
                                        class: "text-base",
                                        style: "color: var(--secondary-color-5);",
                                        "Didn't receive the email? "
                                    }
                                    button {
                                        class: "text-base font-medium hover:underline",
                                        style: "color: var(--focused-border-color);",
                                        onclick: handle_resend,
                                        "Resend link"
                                    }
                                }

                                button {
                                    class: "w-full h-11 rounded-md border text-base font-medium transition-colors",
                                    style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1);",
                                    onclick: move |_| {
                                        router().go_back();
                                    },
                                    "Back to login"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ResetPassword(code: String) -> Element {
    let mut new_password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut is_loading = use_signal(|| false);

    // Get the reset code from URL query params
    // TODO: Parse code from URL query params
    // let code = use_memo(|| {
    //     // Extract code from /reset-password?code={code}
    // });

    let handle_submit = move |ev: Event<FormData>| async move {
        ev.prevent_default();
        is_loading.set(true);

        // TODO: Validate passwords match
        // TODO: Send new password with reset code
        // TODO: On success, redirect to login

        is_loading.set(false);
    };

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center px-4",
            style: "background-color: var(--primary-color);",

            div {
                class: "w-full max-w-md",

                div {
                    class: "rounded-lg border p-8 shadow-sm",
                    style: "background-color: var(--primary-color-1); border-color: var(--primary-color-6);",

                    // Header
                    div {
                        class: "flex flex-col space-y-2 mb-6",
                        h1 {
                            class: "text-2xl font-semibold tracking-tight",
                            style: "color: var(--secondary-color-1);",
                            "Create new password"
                        }
                        p {
                            class: "text-base",
                            style: "color: var(--secondary-color-5);",
                            "Enter a new password for your account"
                        }
                    }

                    // Form
                    form {
                        class: "space-y-5",
                        onsubmit: handle_submit,

                        // New password field
                        div {
                            class: "space-y-2",
                            label {
                                class: "text-base font-medium",
                                style: "color: var(--secondary-color-2);",
                                r#for: "new-password",
                                "New Password"
                            }
                            input {
                                class: "flex h-11 w-full rounded-md border px-4 py-2 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
                                style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1); focus:ring-color: var(--focused-border-color);",
                                r#type: "password",
                                id: "new-password",
                                placeholder: "••••••••",
                                value: "{new_password}",
                                oninput: move |e| new_password.set(e.value()),
                                required: true,
                                minlength: "8",
                                autofocus: true,
                            }
                            p {
                                class: "text-sm",
                                style: "color: var(--secondary-color-5);",
                                "Must be at least 8 characters"
                            }
                        }

                        // Confirm password field
                        div {
                            class: "space-y-2",
                            label {
                                class: "text-base font-medium",
                                style: "color: var(--secondary-color-2);",
                                r#for: "confirm-password",
                                "Confirm New Password"
                            }
                            input {
                                class: "flex h-11 w-full rounded-md border px-4 py-2 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
                                style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1); focus:ring-color: var(--focused-border-color);",
                                r#type: "password",
                                id: "confirm-password",
                                placeholder: "••••••••",
                                value: "{confirm_password}",
                                oninput: move |e| confirm_password.set(e.value()),
                                required: true,
                            }
                        }

                        // Submit button
                        button {
                            class: "w-full h-11 rounded-md text-base font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                            style: "background-color: var(--secondary-color-1); color: var(--primary-color);",
                            r#type: "submit",
                            disabled: is_loading(),
                            if is_loading() {
                                "Resetting password..."
                            } else {
                                "Reset password"
                            }
                        }
                    }
                }
            }
        }
    }
}
