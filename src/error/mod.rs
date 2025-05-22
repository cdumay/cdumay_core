//! Provides structs to manipulate errors
mod builder;
mod convert;
mod error;
mod kind;

pub use builder::ErrorBuilder;
pub use convert::ErrorConverter;
pub use error::Error;
pub use kind::ErrorKind;
