mod stuff;

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
    // let server_addr = std::net::SocketAddr::from_str(&std::env::var("SOCKET").unwrap()).unwrap();
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::new(), stuff::App)
        .merge(server::routes().await);
    axum::serve(
        server::get_custom_listener(addr).await,
        router.into_make_service_with_connect_info::<server::ClientSocket>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::logger::init(tracing::Level::DEBUG).expect("failed to init logger");
    dioxus::launch(stuff::App);
}
