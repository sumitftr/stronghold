use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum RegisterStep {
    NameAndEmail,
    VerifyWithOtp,
    EnterPassword,
    EnterUsername,
}

#[component]
pub fn Register() -> Element {
    let mut current_step = use_signal(|| RegisterStep::NameAndEmail);
    let mut client = use_signal(|| reqwest::Client::builder().build().unwrap_or_default());
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut otp = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(String::new);

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center px-4 py-12 border-[var(--primary-color)]",

            div {
                class: "w-full max-w-md",

                // Progress indicator
                StepIndicator { current_step: current_step() }

                // Card container
                div {
                    class: "rounded-lg border p-8 shadow-sm mt-8 bg-[var(--primary-color-1)] border-[var(--primary-color-6)]",

                    match current_step() {
                        RegisterStep::NameAndEmail => rsx! {
                            super::NameAndEmail {
                                name,
                                email,
                                is_loading,
                                on_submit: move |_| async move {
                                    if let Err(e) = shared::validation::is_email_valid(&email()) {
                                        error_message.set(e.to_string());
                                    }
                                    if let Err(e) = shared::validation::is_display_name_valid(&name()) {
                                        error_message.set(e.to_string());
                                    }
                                    let result = client()
                                        .post(format!("{}/api/register", crate::SERVICE_DOMAIN))
                                        .header(reqwest::header::CONTENT_TYPE, "application/json")
                                        .body(format!(r#"{{"name": "{name}", "email": "{email}"}}"#))
                                        .send().await;
                                    match result {
                                        Ok(v) => {
                                            if v.status().is_success() {
                                                current_step.set(RegisterStep::VerifyWithOtp);
                                            } else {
                                                match v.text().await {
                                                    Ok(e) => error_message.set(e),
                                                    Err(e) => error_message.set(e.to_string())
                                                }

                                            }
                                        },
                                        Err(e) => error_message.set(e.to_string()),
                                    }
                                }
                            }
                        },
                        RegisterStep::VerifyWithOtp => rsx! {
                            super::VerifyWithOtp {
                                email: email,
                                otp,
                                is_loading,
                                on_submit: move |_| async move {
                                    let result = client()
                                        .post(format!("{}/api/register/verify_email", crate::SERVICE_DOMAIN))
                                        .header(reqwest::header::CONTENT_TYPE, "application/json")
                                        .body(format!(r#"{{"email": "{email}", "otp": "{otp}"}}"#))
                                        .send().await;
                                    match result {
                                        Ok(v) => {
                                            if v.status().is_success() {
                                                current_step.set(RegisterStep::EnterPassword);
                                            } else {
                                                match v.text().await {
                                                    Ok(e) => error_message.set(e),
                                                    Err(e) => error_message.set(e.to_string())
                                                }

                                            }
                                        },
                                        Err(e) => error_message.set(e.to_string()),
                                    }
                                },
                                on_resend: move |_| async move {
                                    let result = client()
                                        .post(format!("{}/api/register/resend_otp", crate::SERVICE_DOMAIN))
                                        .header(reqwest::header::CONTENT_TYPE, "application/json")
                                        .body(format!(r#"{{"email": "{email}""#))
                                        .send().await;
                                    match result {
                                        Ok(v) => {
                                            if !v.status().is_success() {
                                                match v.text().await {
                                                    Ok(e) => error_message.set(e),
                                                    Err(e) => error_message.set(e.to_string())
                                                }
                                            }
                                        },
                                        Err(e) => error_message.set(e.to_string()),
                                    }
                                },
                            }
                        },
                        RegisterStep::EnterPassword => rsx! {
                            super::EnterPassword {
                                email: email,
                                password,
                                confirm_password,
                                is_loading,
                                on_submit: move |_| async move {
                                    if password() != confirm_password() {
                                        error_message.set("Passwords doesn't match".to_string());
                                    }
                                    if let Err(e) = shared::validation::is_password_strong(&password()) {
                                        error_message.set(e.to_string());
                                    }
                                    let result = client()
                                        .post(format!("{}/api/register/set_password", crate::SERVICE_DOMAIN))
                                        .header(reqwest::header::CONTENT_TYPE, "application/json")
                                        .body(format!(r#"{{"email": "{email}", "password": "{password}"}}"#))
                                        .send().await;
                                    match result {
                                        Ok(v) => {
                                            if v.status().is_success() {
                                                current_step.set(RegisterStep::EnterUsername);
                                            } else {
                                                match v.text().await {
                                                    Ok(e) => error_message.set(e),
                                                    Err(e) => error_message.set(e.to_string())
                                                }
                                            }
                                        },
                                        Err(e) => error_message.set(e.to_string()),
                                    }
                                },
                            }
                        },
                        RegisterStep::EnterUsername => rsx! {
                            super::EnterUsername {
                                email: email,
                                username,
                                is_loading,
                                on_submit: move |_| async move {
                                    if let Err(e) = shared::validation::is_username_valid(&username()) {
                                        error_message.set(e.to_string());
                                    }
                                    let result = client()
                                        .post(format!("{}/api/register/set_username", crate::SERVICE_DOMAIN))
                                        .header(reqwest::header::CONTENT_TYPE, "application/json")
                                        .body(format!(r#"{{"email": "{email}", "username": "{username}"}}"#))
                                        .send().await;
                                    match result {
                                        Ok(v) => {
                                            if v.status().is_success() {
                                                router().replace(Route::Login {});
                                            } else {
                                                match v.text().await {
                                                    Ok(e) => error_message.set(e),
                                                    Err(e) => error_message.set(e.to_string())
                                                }

                                            }
                                        },
                                        Err(e) => error_message.set(e.to_string()),
                                    }
                                },
                            }
                        },
                    }

                    // Login link
                    div {
                        class: "mt-8 text-center text-base text-[var(--secondary-color-5)]",
                        "Already have an account? "
                        Link {
                            to: Route::Login {},
                            class: "font-medium hover:underline text-[var(--focused-border-color)]",
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
        ("Account", RegisterStep::NameAndEmail),
        ("Verify", RegisterStep::VerifyWithOtp),
        ("Password", RegisterStep::EnterPassword),
        ("Username", RegisterStep::EnterUsername),
    ];

    let current_index = steps.iter().position(|(_, step)| *step == current_step).unwrap_or(0);

    rsx! {
        div {
            class: "w-full",

            // Circles and lines row
            div {
                class: "flex items-center w-full",
                for (index, (label , step)) in steps.iter().enumerate() {
                    // Step circle
                    div {
                        key: "{index}-step",
                        class: "flex justify-center flex-none",
                        div {
                            class: "w-10 h-10 rounded-full flex items-center justify-center text-sm font-medium transition-colors",
                            style: if index <= current_index {
                                "background-color: var(--focused-border-color); color: var(--primary-color);"
                            } else {
                                "background-color: var(--primary-color-5); color: var(--secondary-color-5);"
                            },
                            "{index + 1}"
                        }
                    }

                    // Line connector (after each step except the last)
                    if index < steps.len() - 1 {
                        div {
                            key: "{index}-line",
                            class: "flex-1 h-0.5 mx-2",
                            style: if index < current_index {
                                "background-color: var(--focused-border-color);"
                            } else {
                                "background-color: var(--primary-color-6);"
                            }
                        }
                    }
                }
            }

            // Labels row
            div {
                class: "flex items-center w-full mt-2",
                for (index, (label , step)) in steps.iter().enumerate() {
                    // Step label - fixed width for alignment
                    div {
                        key: "{index}-label",
                        class: "flex justify-center flex-[0_0_40px] w-[40px]",
                        span {
                            class: "text-xs font-medium whitespace-nowrap",
                            style: if index <= current_index {
                                "color: var(--secondary-color-1);"
                            } else {
                                "color: var(--secondary-color-5);"
                            },
                            "{label}"
                        }
                    }

                    // Spacer (matching the line width)
                    if index < steps.len() - 1 {
                        div {
                            key: "{index}-spacer",
                            class: "flex-1 mx-2",
                        }
                    }
                }
            }
        }
    }
}
