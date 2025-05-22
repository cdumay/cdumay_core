//! Provides structs to manipulate errors
mod builder;
mod error;
mod kind;

pub use builder::ErrorBuilder;
pub use error::Error;
pub use kind::ErrorKind;
