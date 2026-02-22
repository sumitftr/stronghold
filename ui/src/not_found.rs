use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NotFound(endpoint: Vec<String>) -> Element {
    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center px-4",
            div {
                class: "max-w-2xl w-full space-y-8 text-center",

                // Icon/Visual element
                div {
                    class: "flex justify-center mb-8",
                    div {
                        class: "relative",
                        // 404 number
                        h1 {
                            class: "text-9xl font-bold tracking-tight",
                            style: "color: var(--primary-color-5);",
                            "404"
                        }
                    }
                }

                // Main heading
                h2 {
                    class: "text-3xl font-semibold tracking-tight mb-4",
                    style: "color: var(--secondary-color-1);",
                    "Page not found"
                }

                // Description
                p {
                    class: "text-lg mb-8",
                    style: "color: var(--secondary-color-5);",
                    "We are sorry, the page you requested doesn't exist."
                }

                // Action buttons
                div {
                    class: "flex gap-4 justify-center flex-wrap mb-8",
                    Link {
                        to: Route::Home {},
                        class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors h-10 px-6 py-2 cursor-pointer",
                        style: "background-color: var(--secondary-color-1); color: var(--primary-color);",
                        "Go back home"
                    }
                    button {
                        onclick: move |_| router().go_back(),
                        class: "inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors h-10 px-6 py-2 border cursor-pointer",
                        style: "border-color: var(--primary-color-6); color: var(--secondary-color-3);",
                        "Go back"
                    }
                }

                // Error details (collapsible)
                details {
                    class: "text-left rounded-lg border p-4 mt-8",
                    style: "background-color: var(--primary-color-3); border-color: var(--primary-color-6);",
                    summary {
                        class: "cursor-pointer text-sm font-medium mb-2",
                        style: "color: var(--secondary-color-5);",
                        "Error Description"
                    }
                    pre {
                        class: "text-xs mt-2 p-3 rounded overflow-x-auto",
                        style: "background-color: var(--primary-color-4); color: var(--secondary-error-color); font-family: 'Courier New', monospace;",
                        "Attempted to navigate to:\n{endpoint:#?}"
                    }
                }
            }
        }
    }
}
