pub mod validation;

pub static SERVICE_NAME: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| std::env::var("SERVICE_NAME").unwrap());

pub static SERVICE_DOMAIN: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| std::env::var("SERVICE_DOMAIN").unwrap());
