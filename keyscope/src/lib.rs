pub use self::errors::Error;

pub mod config;
pub mod errors;
pub mod providers;

pub type Result<T, E = Error> = std::result::Result<T, E>;
