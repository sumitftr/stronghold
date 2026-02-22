mod error;
pub mod generate;
pub mod mail;
pub mod oauth;
pub mod session;

pub use error::AppError;

pub static SECRET_KEY: std::sync::LazyLock<Vec<u8>> =
    std::sync::LazyLock::new(|| Vec::from(std::env::var("SECRET_KEY").unwrap()));
