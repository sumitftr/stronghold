mod image;
mod text;
mod unique;

pub use image::*;
pub use text::*;
pub use unique::*;

#[derive(PartialEq, Debug, thiserror::Error)]
pub enum ValidationError {
    // Name
    #[error("Name should have atleast 2 characters")]
    NameTooShort,
    #[error("Name should be lesser than 64 characters")]
    NameTooLong,
    #[error("{0}")]
    InvalidNameFormat(String),

    // Bio
    #[error("Bio too long (MAX: 3000)")]
    BioTooLong,

    // Country
    #[error("Country not found")]
    CountryNotFound,

    // Gender
    #[error("Invalid Gender")]
    InvalidGender,

    // DateTime
    #[error("Invalid month")]
    InvalidMonth,
    #[error("Invalid UTC offset")]
    InvalidUtcOffset,
    #[error("Date Parse Error: {0}")]
    DateParseError(#[from] time::error::ComponentRange),
    #[error("Invalid Date: {0}")]
    InvalidDate(String),

    // Password
    #[error("Password cannot be less than 8 characters")]
    PasswordTooShort,
    #[error("Password cannot be more than 128 characters")]
    PasswordTooLong,
    #[error("Password must contain a lowercase alphabet, a uppercase alphabet and a digit")]
    InvalidPasswordFormat,

    // Username
    #[error("Invalid Username: {0}")]
    InvalidUsername(String),

    // Email
    #[error("Invalid Email Format")]
    InvalidEmailFormat,

    // Image
    #[error("Image data too short: need at least {needed} bytes, got {got}")]
    ImageTooShort { needed: usize, got: usize },
    #[error("Invalid {format} image: {reason}")]
    InvalidImageFormat { format: String, reason: String },
    #[error("Unknown image format: Only jpg, jpeg, png, webp are allowed")]
    UnknownImageFormat,

    // Fallback
    #[error("Error: {0}")]
    InvalidData(String),
}
