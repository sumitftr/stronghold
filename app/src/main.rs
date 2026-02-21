use dioxus::prelude::*;

const THEME_CSS: Asset = asset!("/assets/components-theme.css");

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::logger::init(tracing::Level::DEBUG).expect("failed to init logger");

    #[cfg(feature = "web")]
    dioxus::LaunchBuilder::web().launch(App);

    #[cfg(feature = "desktop")]
    dioxus::launch(App);

    #[cfg(feature = "mobile")]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "icon",
            href: "https://yt3.ggpht.com/VMFfvP2TUfjGvMfuCzgmUZxoab3pKFMBFIt33vXjbzuYWJV91laJXQ4NC1R32geeFEXFbhQYRw=s600-c-k-c0x00ffffff-no-rj-rp-mo"
        }
        document::Script {
            src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4",
            r#type: "module"
        }
        document::Stylesheet { href: THEME_CSS }
    }
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing::level_filters::LevelFilter::from_level(tracing::Level::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default())
        .init();

    axum::serve(
        server::get_custom_listener().await,
        server::routes().await.into_make_service_with_connect_info::<server::ClientSocket>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
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
