use dioxus::prelude::*;

#[component]
pub(super) fn NameAndEmail(
    name: Signal<String>,
    email: Signal<String>,
    is_loading: Signal<bool>,
    on_submit: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            // Header
            div {
                class: "flex flex-col space-y-2 mb-8",
                h1 {
                    class: "text-3xl font-semibold tracking-tight text-[var(--secondary-color-1)]",
                    "Create account"
                }
                p {
                    class: "text-base text-[var(--secondary-color-5)]",
                    "Enter your details to get started"
                }
            }

            // Form
            form {
                class: "space-y-5",
                onsubmit: move |ev: Event<FormData>| {
                    ev.prevent_default();
                    on_submit.call(());
                },

                // Name field
                div {
                    class: "space-y-2",
                    label {
                        class: "text-base font-medium text-[var(--secondary-color-2)]",
                        r#for: "name",
                        "Full Name"
                    }
                    input {
                        class: "flex h-11 w-full rounded-md border px-4 py-2 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1); focus:ring-color: var(--focused-border-color);",
                        r#type: "text",
                        id: "name",
                        placeholder: "Your Name",
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                        required: true,
                    }
                }

                // Email field
                div {
                    class: "space-y-2",
                    label {
                        class: "text-base font-medium text-[var(--secondary-color-2)]",
                        r#for: "email",
                        "Email Address"
                    }
                    input {
                        class: "flex h-11 w-full rounded-md border px-4 py-2 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 text-[var(--secondary-color-1)]",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); focus:ring-color: var(--focused-border-color);",
                        r#type: "email",
                        id: "email",
                        placeholder: "name@example.com",
                        value: "{email}",
                        oninput: move |e| email.set(e.value()),
                        required: true,
                    }
                }

                // Submit button
                button {
                    class: "w-full h-11 rounded-md text-base font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed text-[var(--primary-color)] bg-[var(--secondary-color-1)]",
                    r#type: "submit",
                    disabled: is_loading(),
                    if is_loading() {
                        "Creating account..."
                    } else {
                        "Continue"
                    }
                }
            }
        }
    }
}

#[component]
pub(super) fn VerifyWithOtp(
    email: ReadSignal<String>,
    otp: Signal<String>,
    is_loading: Signal<bool>,
    on_submit: EventHandler<()>,
    on_resend: EventHandler<()>,
) -> Element {
    let mut resend_cooldown = use_signal(|| 0);

    // TODO: Implement resend cooldown timer

    rsx! {
        div {
            // Header
            div {
                class: "flex flex-col space-y-2 mb-8",
                h1 {
                    class: "text-3xl font-semibold tracking-tight text-[var(--secondary-color-1)]",
                    "Verify your email"
                }
                p {
                    class: "text-base text-[var(--secondary-color-5)]",
                    "We've sent a verification code to"
                }
                p {
                    class: "text-base font-medium text-[var(--secondary-color-1)]",
                    "{email()}"
                }
            }

            // Form
            form {
                class: "space-y-5",
                onsubmit: move |ev: Event<FormData>| {
                    ev.prevent_default();
                    on_submit.call(());
                },

                // OTP field
                div {
                    class: "space-y-2",
                    label {
                        class: "text-base font-medium text-[var(--secondary-color-2)]",
                        r#for: "otp",
                        "Verification Code"
                    }
                    input {
                        class: "flex h-11 w-full rounded-md border px-4 py-2 text-base text-center tracking-widest transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 text-[var(--secondary-color-1)] bg-[var(--primary-color-3)]",
                        style: "border-color: var(--primary-color-6); focus:ring-color: var(--focused-border-color); font-size: 1.25rem; letter-spacing: 0.5em;",
                        r#type: "text",
                        id: "otp",
                        placeholder: "000000",
                        value: "{otp}",
                        oninput: move |e| otp.set(e.value()),
                        required: true,
                        maxlength: "6",
                        pattern: "[0-9]*",
                    }
                }

                // Resend button
                div {
                    class: "text-center",
                    button {
                        class: "text-base font-medium hover:underline disabled:opacity-50 disabled:no-underline bg-[var(--focused-border-color)]",
                        r#type: "button",
                        onclick: move |_| on_resend.call(()),
                        disabled: resend_cooldown() > 0,
                        if resend_cooldown() > 0 {
                            "Resend code in {resend_cooldown()}s"
                        } else {
                            "Resend verification code"
                        }
                    }
                }

                // Action buttons
                // div {
                //     class: "flex gap-3 pt-2",
                //     button {
                //         class: "flex-1 h-11 rounded-md border text-base font-medium transition-colors text-[var(--secondary-color-1)] bg-[var(--primary-color-3)] border-[var(--primary-color-6)]",
                //         r#type: "button",
                //         onclick: move |_| on_back.call(()),
                //         "Back"
                //     }
                button {
                    class: "flex-1 h-11 rounded-md text-base font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed text-[var(--primary-color)]",
                    style: "background-color: var(--secondary-color-1);",
                    r#type: "submit",
                    disabled: is_loading(),
                    if is_loading() {
                        "Verifying..."
                    } else {
                        "Verify"
                    }
                }
                // }
            }
        }
    }
}

#[component]
pub(super) fn EnterPassword(
    email: ReadSignal<String>,
    password: Signal<String>,
    confirm_password: Signal<String>,
    is_loading: Signal<bool>,
    on_submit: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            // Header
            div {
                class: "flex flex-col space-y-2 mb-8",
                h1 {
                    class: "text-3xl font-semibold tracking-tight bg-[var(--secondary-color-1)]",
                    "Set your password"
                }
                p {
                    class: "text-base text-[var(--secondary-color-5)]",
                    "Choose a strong password for your account"
                }
            }

            // Form
            form {
                class: "space-y-5",
                onsubmit: move |ev: Event<FormData>| {
                    ev.prevent_default();
                    on_submit.call(())
                },

                // Password field
                div {
                    class: "space-y-2",
                    label {
                        class: "text-base font-medium text-[var(--secondary-color-2)]",
                        r#for: "password",
                        "Password"
                    }
                    input {
                        class: "flex h-11 w-full rounded-md border px-4 py-2 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 text-[var(--secondary-color-1)]",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); focus:ring-color: var(--focused-border-color);",
                        r#type: "password",
                        id: "password",
                        placeholder: "••••••••",
                        value: "{password}",
                        oninput: move |e| password.set(e.value()),
                        required: true,
                        minlength: "8",
                    }
                    p {
                        class: "text-sm text-[var(--secondary-color-5)]",
                        "Must be at least 8 characters"
                    }
                }

                // Confirm password field
                div {
                    class: "space-y-2",
                    label {
                        class: "text-base font-medium text-[var(--secondary-color-2)]",
                        r#for: "confirm-password",
                        "Confirm Password"
                    }
                    input {
                        class: "flex h-11 w-full rounded-md border px-4 py-2 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 text-[var(--secondary-color-1)]",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); focus:ring-color: var(--focused-border-color);",
                        r#type: "password",
                        id: "confirm-password",
                        placeholder: "••••••••",
                        value: "{confirm_password}",
                        oninput: move |e| confirm_password.set(e.value()),
                        required: true,
                    }
                }

                // Action buttons
                // div {
                //     class: "flex gap-3 pt-2",
                //     button {
                //         class: "flex-1 h-11 rounded-md border text-base font-medium transition-colors text-[var(--secondary-color-1)]",
                //         style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6);",
                //         r#type: "button",
                //         onclick: move |_| on_back.call(()),
                //         "Back"
                //     }
                button {
                    class: "flex-1 h-11 rounded-md text-base font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed text-[var(--primary-color)]",
                    style: "background-color: var(--secondary-color-1);",
                    r#type: "submit",
                    disabled: is_loading(),
                    if is_loading() {
                        "Setting password..."
                    } else {
                        "Continue"
                    }
                }
                // }
            }
        }
    }
}

#[component]
pub(super) fn EnterUsername(
    email: ReadSignal<String>,
    username: Signal<String>,
    is_loading: Signal<bool>,
    on_submit: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            // Header
            div {
                class: "flex flex-col space-y-2 mb-8",
                h1 {
                    class: "text-3xl font-semibold tracking-tight text-[var(--secondary-color-1)]",
                    "Choose your username"
                }
                p {
                    class: "text-base text-[var(--secondary-color-5)]",
                    "This is how others will see you on the platform"
                }
            }

            // Form
            form {
                class: "space-y-5",
                onsubmit: move |ev: Event<FormData>| {
                    ev.prevent_default();
                    on_submit.call(())
                },

                // Username field
                div {
                    class: "space-y-2",
                    label {
                        class: "text-base font-medium text-[var(--secondary-color-2)]",
                        r#for: "username",
                        "Username"
                    }
                    input {
                        class: "flex h-11 w-full rounded-md border px-4 py-2 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 text-[var(--secondary-color-1)]",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); focus:ring-color: var(--focused-border-color);",
                        r#type: "text",
                        id: "username",
                        value: "{username}",
                        oninput: move |e| username.set(e.value()),
                        required: true,
                        pattern: "[a-zA-Z0-9.]+",
                        minlength: "6",
                    }
                    p {
                        class: "text-sm text-[var(--secondary-color-5)]",
                        "Only letters, numbers, and periods. Min 6 characters."
                    }
                }

                // Action buttons
                // div {
                //     class: "flex gap-3 pt-2",
                //     button {
                //         class: "flex-1 h-11 rounded-md border text-base font-medium transition-colors text-[var(--secondary-color-1)] bg-[var(--secondary-color-3)]",
                //         style: "border-color: var(--primary-color-6);",
                //         r#type: "button",
                //         onclick: move |_| on_back.call(()),
                //         "Back"
                //     }
                button {
                    class: "flex-1 h-11 rounded-md text-base font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed text-[var(--primary-color)] bg-[var(--secondary-color-1)]",
                    r#type: "submit",
                    disabled: is_loading(),
                    if is_loading() {
                        "Completing registration..."
                    } else {
                        "Complete Registration"
                    }
                }
                // }
            }
        }
    }
}
