use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    const THEME_CSS: Asset = asset!("/assets/components-theme.css");

    rsx! {
        document::Link {
            rel: "icon",
            href: "https://cdn.simpleicons.org/rust"
        }
        document::Script {
            r#async: true,
            src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4",
            r#type: "module"
        }
        document::Stylesheet { href: THEME_CSS }

        Router::<ui::Route> {}
    }
}

/// Shutdown signal to run axum with graceful shutdown when
/// a user presses Ctrl+C or Unix sends a terminate signal.
#[cfg(feature = "server")]
pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
