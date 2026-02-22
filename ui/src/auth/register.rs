use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum RegisterStep {
    CreateAccount,
    VerifyEmail,
    SetPassword,
    SetUsername,
}

#[component]
pub fn Register() -> Element {
    let mut current_step = use_signal(|| RegisterStep::CreateAccount);
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut otp = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut is_loading = use_signal(|| false);

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center px-4 py-12",
            style: "background-color: var(--primary-color);",

            div {
                class: "w-full max-w-md",

                // Progress indicator
                StepIndicator { current_step: current_step() }

                // Card container
                div {
                    class: "rounded-lg border p-8 shadow-sm mt-8",
                    style: "background-color: var(--primary-color-1); border-color: var(--primary-color-6);",

                    match current_step() {
                        RegisterStep::CreateAccount => rsx! {
                            CreateAccountStep {
                                name,
                                email,
                                is_loading,
                                on_next: move |_| {
                                    // TODO: Send CreateUserRequest
                                    current_step.set(RegisterStep::VerifyEmail);
                                }
                            }
                        },
                        RegisterStep::VerifyEmail => rsx! {
                            VerifyEmailStep {
                                email: email(),
                                otp,
                                is_loading,
                                on_next: move |_| {
                                    // TODO: Send VerifyEmailRequest
                                    current_step.set(RegisterStep::SetPassword);
                                },
                                on_resend: move |_| {
                                    // TODO: Send ResendOtpRequest
                                },
                                on_back: move |_| {
                                    current_step.set(RegisterStep::CreateAccount);
                                }
                            }
                        },
                        RegisterStep::SetPassword => rsx! {
                            SetPasswordStep {
                                email: email(),
                                password,
                                confirm_password,
                                is_loading,
                                on_next: move |_| {
                                    // TODO: Send SetPasswordRequest
                                    current_step.set(RegisterStep::SetUsername);
                                },
                                on_back: move |_| {
                                    current_step.set(RegisterStep::VerifyEmail);
                                }
                            }
                        },
                        RegisterStep::SetUsername => rsx! {
                            SetUsernameStep {
                                email: email(),
                                username,
                                is_loading,
                                on_complete: move |_| {
                                    // TODO: Send SetUsernameRequest
                                    // TODO: Redirect to login or dashboard
                                },
                                on_back: move |_| {
                                    current_step.set(RegisterStep::SetPassword);
                                }
                            }
                        },
                    }

                    // Login link
                    div {
                        class: "mt-8 text-center text-base",
                        style: "color: var(--secondary-color-5);",
                        "Already have an account? "
                        Link {
                            to: Route::Login {},
                            class: "font-medium hover:underline",
                            style: "color: var(--focused-border-color);",
                            "Login"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StepIndicator(current_step: RegisterStep) -> Element {
    let steps = [
        ("Account", RegisterStep::CreateAccount),
        ("Verify", RegisterStep::VerifyEmail),
        ("Password", RegisterStep::SetPassword),
        ("Username", RegisterStep::SetUsername),
    ];

    let current_index = steps
        .iter()
        .position(|(_, step)| *step == current_step)
        .unwrap_or(0);

    rsx! {
        div {
            class: "flex items-center justify-between",
            for (index, (label , step)) in steps.iter().enumerate() {
                div {
                    key: "{index}",
                    class: "flex items-center",
                    class: if index > 0 { "flex-1" } else { "" },

                    // Line connector
                    if index > 0 {
                        div {
                            class: "flex-1 h-0.5 mx-2",
                            style: if index <= current_index {
                                "background-color: var(--focused-border-color);"
                            } else {
                                "background-color: var(--primary-color-6);"
                            }
                        }
                    }

                    // Step circle and label
                    div {
                        class: "flex flex-col items-center",
                        div {
                            class: "w-10 h-10 rounded-full flex items-center justify-center text-sm font-medium transition-colors",
                            style: if index <= current_index {
                                "background-color: var(--focused-border-color); color: var(--primary-color);"
                            } else {
                                "background-color: var(--primary-color-5); color: var(--secondary-color-5);"
                            },
                            "{index + 1}"
                        }
                        span {
                            class: "text-xs mt-2 font-medium",
                            style: if index <= current_index {
                                "color: var(--secondary-color-1);"
                            } else {
                                "color: var(--secondary-color-5);"
                            },
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CreateAccountStep(
    name: Signal<String>,
    email: Signal<String>,
    is_loading: Signal<bool>,
    on_next: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            // Header
            div {
                class: "flex flex-col space-y-2 mb-8",
                h1 {
                    class: "text-3xl font-semibold tracking-tight",
                    style: "color: var(--secondary-color-1);",
                    "Create account"
                }
                p {
                    class: "text-base",
                    style: "color: var(--secondary-color-5);",
                    "Enter your details to get started"
                }
            }

            // Form
            form {
                class: "space-y-5",
                onsubmit: move |ev: Event<FormData>| {
                    ev.prevent_default();
                    // TODO: send request to /api/register/start
                    on_next.call(());
                },

                // Name field
                div {
                    class: "space-y-2",
                    label {
                        class: "text-base font-medium",
                        style: "color: var(--secondary-color-2);",
                        r#for: "name",
                        "Full Name"
                    }
                    input {
                        class: "flex h-11 w-full rounded-md border px-4 py-2 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1); focus:ring-color: var(--focused-border-color);",
                        r#type: "text",
                        id: "name",
                        placeholder: "John Doe",
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                        required: true,
                    }
                }

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
                    }
                }

                // Submit button
                button {
                    class: "w-full h-11 rounded-md text-base font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                    style: "background-color: var(--secondary-color-1); color: var(--primary-color);",
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
fn VerifyEmailStep(
    email: String,
    otp: Signal<String>,
    is_loading: Signal<bool>,
    on_next: EventHandler<()>,
    on_resend: EventHandler<()>,
    on_back: EventHandler<()>,
) -> Element {
    let mut resend_cooldown = use_signal(|| 0);

    // TODO: Implement resend cooldown timer

    rsx! {
        div {
            // Header
            div {
                class: "flex flex-col space-y-2 mb-8",
                h1 {
                    class: "text-3xl font-semibold tracking-tight",
                    style: "color: var(--secondary-color-1);",
                    "Verify your email"
                }
                p {
                    class: "text-base",
                    style: "color: var(--secondary-color-5);",
                    "We've sent a verification code to"
                }
                p {
                    class: "text-base font-medium",
                    style: "color: var(--secondary-color-1);",
                    "{email}"
                }
            }

            // Form
            form {
                class: "space-y-5",
                onsubmit: move |ev: Event<FormData>| {
                    ev.prevent_default();
                    // TODO: send request to /api/register/verify_email
                    on_next.call(());
                },

                // OTP field
                div {
                    class: "space-y-2",
                    label {
                        class: "text-base font-medium",
                        style: "color: var(--secondary-color-2);",
                        r#for: "otp",
                        "Verification Code"
                    }
                    input {
                        class: "flex h-11 w-full rounded-md border px-4 py-2 text-base text-center tracking-widest transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1); focus:ring-color: var(--focused-border-color); font-size: 1.25rem; letter-spacing: 0.5em;",
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
                        class: "text-base font-medium hover:underline disabled:opacity-50 disabled:no-underline",
                        style: "color: var(--focused-border-color);",
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
                div {
                    class: "flex gap-3 pt-2",
                    button {
                        class: "flex-1 h-11 rounded-md border text-base font-medium transition-colors",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1);",
                        r#type: "button",
                        onclick: move |_| on_back.call(()),
                        "Back"
                    }
                    button {
                        class: "flex-1 h-11 rounded-md text-base font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                        style: "background-color: var(--secondary-color-1); color: var(--primary-color);",
                        r#type: "submit",
                        disabled: is_loading(),
                        if is_loading() {
                            "Verifying..."
                        } else {
                            "Verify"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SetPasswordStep(
    email: String,
    password: Signal<String>,
    confirm_password: Signal<String>,
    is_loading: Signal<bool>,
    on_next: EventHandler<()>,
    on_back: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            // Header
            div {
                class: "flex flex-col space-y-2 mb-8",
                h1 {
                    class: "text-3xl font-semibold tracking-tight",
                    style: "color: var(--secondary-color-1);",
                    "Set your password"
                }
                p {
                    class: "text-base",
                    style: "color: var(--secondary-color-5);",
                    "Choose a strong password for your account"
                }
            }

            // Form
            form {
                class: "space-y-5",
                onsubmit: move |ev: Event<FormData>| {
                    ev.prevent_default();
                    // TODO: send request to /api/register/set_password
                    on_next.call(())
                },

                // Password field
                div {
                    class: "space-y-2",
                    label {
                        class: "text-base font-medium",
                        style: "color: var(--secondary-color-2);",
                        r#for: "password",
                        "Password"
                    }
                    input {
                        class: "flex h-11 w-full rounded-md border px-4 py-2 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1); focus:ring-color: var(--focused-border-color);",
                        r#type: "password",
                        id: "password",
                        placeholder: "••••••••",
                        value: "{password}",
                        oninput: move |e| password.set(e.value()),
                        required: true,
                        minlength: "8",
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
                        "Confirm Password"
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

                // Action buttons
                div {
                    class: "flex gap-3 pt-2",
                    button {
                        class: "flex-1 h-11 rounded-md border text-base font-medium transition-colors",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1);",
                        r#type: "button",
                        onclick: move |_| on_back.call(()),
                        "Back"
                    }
                    button {
                        class: "flex-1 h-11 rounded-md text-base font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                        style: "background-color: var(--secondary-color-1); color: var(--primary-color);",
                        r#type: "submit",
                        disabled: is_loading(),
                        if is_loading() {
                            "Setting password..."
                        } else {
                            "Continue"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SetUsernameStep(
    email: String,
    username: Signal<String>,
    is_loading: Signal<bool>,
    on_complete: EventHandler<()>,
    on_back: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            // Header
            div {
                class: "flex flex-col space-y-2 mb-8",
                h1 {
                    class: "text-3xl font-semibold tracking-tight",
                    style: "color: var(--secondary-color-1);",
                    "Choose your username"
                }
                p {
                    class: "text-base",
                    style: "color: var(--secondary-color-5);",
                    "This is how others will see you on the platform"
                }
            }

            // Form
            form {
                class: "space-y-5",
                onsubmit: move |ev: Event<FormData>| {
                    ev.prevent_default();
                    // TODO: send request to /api/register/set_username
                    on_complete.call(())
                },

                // Username field
                div {
                    class: "space-y-2",
                    label {
                        class: "text-base font-medium",
                        style: "color: var(--secondary-color-2);",
                        r#for: "username",
                        "Username"
                    }
                    input {
                        class: "flex h-11 w-full rounded-md border px-4 py-2 text-base transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1); focus:ring-color: var(--focused-border-color);",
                        r#type: "text",
                        id: "username",
                        placeholder: "johndoe",
                        value: "{username}",
                        oninput: move |e| username.set(e.value()),
                        required: true,
                        pattern: "[a-zA-Z0-9_]+",
                        minlength: "3",
                    }
                    p {
                        class: "text-sm",
                        style: "color: var(--secondary-color-5);",
                        "Only letters, numbers, and underscores. Min 3 characters."
                    }
                }

                // Action buttons
                div {
                    class: "flex gap-3 pt-2",
                    button {
                        class: "flex-1 h-11 rounded-md border text-base font-medium transition-colors",
                        style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6); color: var(--secondary-color-1);",
                        r#type: "button",
                        onclick: move |_| on_back.call(()),
                        "Back"
                    }
                    button {
                        class: "flex-1 h-11 rounded-md text-base font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                        style: "background-color: var(--secondary-color-1); color: var(--primary-color);",
                        r#type: "submit",
                        disabled: is_loading(),
                        if is_loading() {
                            "Completing registration..."
                        } else {
                            "Complete Registration"
                        }
                    }
                }
            }
        }
    }
}
