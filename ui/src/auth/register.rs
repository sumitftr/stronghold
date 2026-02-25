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
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut otp = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut username = use_signal(String::new);
    let mut is_loading = use_signal(|| false);

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
                                on_next: move |_| {
                                    // TODO: Send CreateUserRequest
                                    current_step.set(RegisterStep::VerifyWithOtp);
                                }
                            }
                        },
                        RegisterStep::VerifyWithOtp => rsx! {
                            super::VerifyWithOtp {
                                email: email,
                                otp,
                                is_loading,
                                on_next: move |_| {
                                    // TODO: Send VerifyEmailRequest
                                    current_step.set(RegisterStep::EnterPassword);
                                },
                                on_resend: move |_| {
                                    // TODO: Send ResendOtpRequest
                                },
                                on_back: move |_| {
                                    current_step.set(RegisterStep::NameAndEmail);
                                }
                            }
                        },
                        RegisterStep::EnterPassword => rsx! {
                            super::EnterPassword {
                                email: email,
                                password,
                                confirm_password,
                                is_loading,
                                on_next: move |_| {
                                    // TODO: Send SetPasswordRequest
                                    current_step.set(RegisterStep::EnterUsername);
                                },
                                on_back: move |_| {
                                    current_step.set(RegisterStep::VerifyWithOtp);
                                }
                            }
                        },
                        RegisterStep::EnterUsername => rsx! {
                            super::EnterUsername {
                                email: email,
                                username,
                                is_loading,
                                on_complete: move |_| {
                                    // TODO: Send SetUsernameRequest
                                    // TODO: Redirect to login or dashboard
                                },
                                on_back: move |_| {
                                    current_step.set(RegisterStep::EnterPassword);
                                }
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
