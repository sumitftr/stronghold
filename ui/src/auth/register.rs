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
                            super::NameAndEmail {
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
                            super::VerifyWithOtp {
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
                            super::EnterPassword {
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
                            super::EnterUsername {
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

    let current_index = steps.iter().position(|(_, step)| *step == current_step).unwrap_or(0);

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
