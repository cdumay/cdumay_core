/// A builder for constructing detailed and structured `Error` instances.
///
/// `ErrorBuilder` provides a fluent interface to build a custom `Error`
/// with optional HTTP code, message, and additional details.
///
/// This pattern is useful when you want to customize errors
/// returned from your application, especially for APIs.
///
/// # Example
/// ```
/// use cdumay_core::{ErrorBuilder, ErrorKind};
/// use serde_value::Value;
/// use std::collections::BTreeMap;
///
/// let mut details = BTreeMap::new();
/// details.insert("field".into(), Value::String("username".into()));
///
/// let kind = ErrorKind("ValidationError", 400, "Invalid input");
///
/// let error = ErrorBuilder::new(kind, "MissingAuth")
///     .with_code(400)
///     .with_message("Username is required".to_string())
///     .with_details(details)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ErrorBuilder {
    /// The high-level classification of the error (e.g. NotFound, FileError).
    kind: crate::error::ErrorKind,
    /// Optional HTTP-like status code (e.g. 400, 500).
    code: Option<u16>,
    /// A unique, contextual name for the error (e.g. "InvalidInput").
    name: String,
    /// Optional human-readable message.
    message: Option<String>,
    /// Optional structured details to include with the error.
    details: std::collections::BTreeMap<String, serde_value::Value>,
}

impl ErrorBuilder {
    /// Creates a new `ErrorBuilder` with a required `ErrorKind` and name.
    ///
    /// # Arguments
    /// * `kind` - The kind/category of the error.
    /// * `name` - A short identifier for the specific error case.
    ///
    /// # Example
    /// ```
    /// use cdumay_core::{ErrorBuilder, ErrorKind};
    ///
    /// let kind = ErrorKind("ValidationError", 400, "Invalid input");
    /// let builder = ErrorBuilder::new(kind, "MissingField");
    /// ```
    pub fn new(kind: crate::error::ErrorKind, name: &str) -> Self {
        Self {
            kind,
            name: name.to_string(),
            code: None,
            message: None,
            details: std::collections::BTreeMap::new(),
        }
    }

    /// Adds a custom status code to the error.
    ///
    /// # Example
    /// ```
    /// use cdumay_core::{ErrorBuilder, ErrorKind};
    ///
    /// let kind = ErrorKind("ValidationError", 400, "Invalid input");
    /// let builder = ErrorBuilder::new(kind, "MissingField").with_code(404);
    /// ```
    pub fn with_code(mut self, code: u16) -> Self {
        self.code = Some(code);
        self
    }

    /// Adds a custom message to the error.
    ///
    /// # Example
    /// ```
    /// use cdumay_core::{ErrorBuilder, ErrorKind};
    ///
    /// let kind = ErrorKind("NotFound", 404, "Not Found");
    /// let builder = ErrorBuilder::new(kind, "UrlDoesNotExists").with_message("Resource not found".to_string());
    /// ```
    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// Adds a structured map of additional error details.
    ///
    /// # Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use serde_value::Value;
    /// use cdumay_core::{ErrorBuilder, ErrorKind};
    ///
    /// let kind = ErrorKind("ValidationError", 400, "Invalid input");
    /// let mut details = BTreeMap::new();
    /// details.insert("reason".into(), Value::String("Invalid ID".into()));
    /// let builder = ErrorBuilder::new(kind, "InvalidField").with_details(details);
    /// ```
    pub fn with_details(mut self, details: std::collections::BTreeMap<String, serde_value::Value>) -> Self {
        self.details = details;
        self
    }

    /// Finalizes the builder and constructs an `Error`.
    ///
    /// If no message or code is provided, it falls back to defaults from the `ErrorKind`.
    ///
    /// # Example
    /// ```
    /// use cdumay_core::{ErrorBuilder, ErrorKind};
    ///
    /// let kind = ErrorKind("ValidationError", 400, "Invalid input");
    /// let error = ErrorBuilder::new(kind, "InvalidField").build();
    /// ```
    pub fn build(self) -> crate::error::Error {
        crate::error::Error::new(
            self.code.unwrap_or(self.kind.code()),
            format!("{}::{}::{}", self.kind.side(), self.kind.name(), self.name),
            self.message.unwrap_or(self.kind.description().to_string()),
            self.details,
        )
    }
}

impl Default for ErrorBuilder {
    /// Creates a default instance of `ErrorBuilder`.
    ///
    /// This implementation of the `Default` trait provides a default error
    /// of type `Error`, with predefined values:
    /// - **Kind**: Represents an internal server error (`InternalServerError`, 500, "Internal Server Error").
    /// - **Class**: Describes the error as a server-side internal error (`Server::InternalServerError::UnknownError`).
    /// - **Message**: The human-readable error message ("Internal Server Error").
    /// - **Details**: No additional error details are provided (`None`).
    ///
    /// This can be used when you need a generic error with standard values.
    ///
    /// # Example
    /// ```
    /// use cdumay_core::ErrorBuilder;
    ///
    /// let error = ErrorBuilder::default().build();
    /// assert_eq!(format!("{}", error), "Server::InternalServerError::UnknownError (500) - Internal Server Error");
    /// ```
    fn default() -> Self {
        ErrorBuilder::new(
            crate::error::ErrorKind("InternalServerError", 500, "Internal Server Error"),
            "UnknownError".into(),
        )
    }
}
