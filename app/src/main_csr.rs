mod stuff;

#[cfg(feature = "server")]
fn main() {
    dioxus::serve(|| async move {
        dotenv::dotenv().ok();
        dioxus::fullstack::set_server_url(&shared::SERVICE_DOMAIN);
        let mut router = dioxus::server::router(stuff::App).merge(server::routes().await);
        Ok(router)
    });
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::logger::init(tracing::Level::DEBUG).expect("failed to init logger");
    dioxus::launch(stuff::App);
}
