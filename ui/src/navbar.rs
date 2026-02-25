use crate::Route;
use components::navbar::*;
use dioxus::prelude::*;

#[component]
pub fn NavigationBar() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-screen overflow-hidden",

            // TODO: Some update component

            // Navigation bar
            div {
                class: "sticky top-0 z-50 w-full border-b flex-shrink-0",
                style: "background-color: var(--primary-color); border-color: var(--primary-color-6); backdrop-filter: blur(8px);",

                nav {
                    class: "px-4 h-16 flex items-center justify-between",

                    // Logo/Brand section
                    div {
                        class: "flex items-center gap-8",

                        Link {
                            to: Route::Home {},
                            class: "text-xl font-semibold tracking-tight",
                            style: "color: var(--secondary-color-1);",
                            "{crate::SERVICE_NAME}"
                        }

                        // Navigation items
                        Navbar {
                            class: "hidden md:flex items-center gap-1",
                            aria_label: "Navbar",
                            NavbarItem {
                                index: 0usize,
                                value: "Blog".to_string(),
                                to: Route::Blog {},
                                class: "px-3 py-2 rounded-md text-sm font-medium transition-colors",
                                "Blog"
                            }
                            NavbarItem {
                                index: 1usize,
                                value: "About".to_string(),
                                to: Route::About {},
                                class: "px-3 py-2 rounded-md text-sm font-medium transition-colors",
                                "About"
                            }
                            // NavbarNav {
                            //     index: 0usize,
                            //     NavbarTrigger { "Inputs" }
                            //     NavbarContent {
                            //         class: "navbar-content",
                            //         NavbarItem {
                            //             index: 0usize,
                            //             value: "Foo".to_string(),
                            //             to: Route::Foo {},
                            //             "Foo"
                            //         }
                            //         NavbarItem {
                            //             index: 1usize,
                            //             value: "Bar".to_string(),
                            //             to: Route::Bar {},
                            //             "Bar"
                            //         }
                            //     }
                            // }
                        }
                    }

                    // Right side actions (optional - you can remove if not needed)
                    div {
                        class: "flex justify-end items-center gap-4",

                        // Login button
                        Link {
                            class: "px-3 py-2 rounded-md text-sm font-medium transition-colors",
                            style: "color: var(--secondary-color-4); hover:background-color: var(--primary-color-5);",
                            aria_label: "Login",
                            to: Route::Login {},
                            "Login"
                        }
                    }
                }
            }

            // Main content
            main {
                class: format!("relative flex flex-1 flex-shrink-0 min-h-0 justify-center items-start md:items-center w-full {}",
                    match router().current() {
                        Route::Home {} => { "overflow-scroll" }
                        _ => { "overflow-x-hidden overflow-y-auto md:overflow-hidden" }
                    }
                ),
                Outlet::<Route> {}
            }
        }
    }
}
