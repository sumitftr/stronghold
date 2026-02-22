use dioxus::prelude::*;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use dioxus::prelude::{DioxusRouterExt, ServeConfig};
    // use std::str::FromStr;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing::level_filters::LevelFilter::from_level(tracing::Level::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default())
        .init();
    let addr = dioxus::cli_config::fullstack_address_or_localhost();
    // let addr = std::net::SocketAddr::from_str(&std::env::var("SOCKET").unwrap()).unwrap();
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::new(), App)
        .merge(server::routes().await);
    axum::serve(
        server::get_custom_listener(addr).await,
        router.into_make_service_with_connect_info::<server::ClientSocket>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

#[component]
fn App() -> Element {
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

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::logger::init(tracing::Level::DEBUG).expect("failed to init logger");

    #[feature(any(feature = "web", feature = "desktop", feature = "mobile"))]
    dioxus::launch(App);

    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    #[cfg(feature = "backend")]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(launch_server());
    }

    // #[cfg(feature = "server")]
    // dioxus::serve(|| async move {
    //     dotenv::dotenv().ok();
    //     let mut router = dioxus::server::router(App);
    //     Ok(router.merge(server::routes().await))
    // });
}

#[cfg(feature = "backend")]
async fn launch_server() {
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
#[cfg(any(feature = "server", feature = "backend"))]
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
