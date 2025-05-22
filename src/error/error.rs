#[cfg(feature = "utoipa")]
use serde_json::json;

/// A structured error type with categorized information.
///
/// The `Error` struct represents an error with a specific kind, classification,
/// message, and optional additional details.
///
/// This structure is designed to facilitate error handling by providing
/// detailed information that can be logged or displayed.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Error
pub struct Error {
    #[serde(skip_serializing)]
    /// Error code
    code: u16,
    /// Error class
    #[cfg_attr(feature = "utoipa", schema(example = "Client::ConfigurationError::InvalidConfiguration"))]
    class: String,
    /// Human-readable message
    #[cfg_attr(feature = "utoipa", schema(example = "Invalid configuration"))]
    message: String,
    /// metadata for internationalization
    #[cfg_attr(feature = "utoipa", schema(example = json!({ "msg": "Missing value for LOG_CLUSTER" })))]
    details: std::collections::BTreeMap<String, serde_value::Value>,
}

impl Error {
    /// Creates a new `Error` instance.
    ///
    /// # Arguments
    ///
    /// * `code` - A numerical status or error code (e.g., HTTP status code).
    /// * `class` - A string representing the error category or type (e.g., "ValidationError").
    /// * `message` - A human-readable error message.
    /// * `details` - Additional error details stored in a key-value map, using `serde_value::Value`.
    ///
    /// # Returns
    ///
    /// A new instance of `Error`.
    ///
    /// # Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use serde_value::Value;
    /// use cdumay_core::Error;
    ///
    /// let mut details = BTreeMap::new();
    /// details.insert("field".to_string(), Value::String("username".to_string()));
    ///
    /// let err = Error::new(400, "ValidationError".to_string(), "Invalid username".to_string(), details);
    /// ```
    pub fn new(code: u16, class: String, message: String, details: std::collections::BTreeMap<String, serde_value::Value>) -> Self {
        Self {
            code,
            class,
            message,
            details,
        }
    }

    /// Returns the numeric error code.
    ///
    /// # Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use cdumay_core::Error;
    ///
    /// let err = Error::new(400, "ValidationError".to_string(), "Invalid username".to_string(), BTreeMap::new());
    /// assert_eq!(err.code(), 400);
    /// ```
    pub fn code(&self) -> u16 {
        self.code
    }

    /// Returns the error class as a `String`.
    ///
    /// # Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use cdumay_core::Error;
    ///
    /// let err = Error::new(400, "ValidationError".to_string(), "Invalid username".to_string(), BTreeMap::new());
    /// assert_eq!(err.class(), "ValidationError");
    /// ```
    pub fn class(&self) -> String {
        self.class.to_string()
    }

    /// Returns the error message as a `String`.
    ///
    /// # Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use cdumay_core::Error;
    ///
    /// let err = Error::new(400, "ValidationError".to_string(), "Invalid username".to_string(), BTreeMap::new());
    /// assert_eq!(err.message(), "Invalid username");
    /// ```
    pub fn message(&self) -> String {
        self.message.to_string()
    }

    /// Returns a clone of the details map.
    ///
    /// # Example
    /// ```
    /// use std::collections::BTreeMap;
    /// use serde_value::Value;
    /// use cdumay_core::Error;
    ///
    /// let mut details = BTreeMap::new();
    /// details.insert("field".to_string(), Value::String("username".to_string()));
    ///
    /// let err = Error::new(400, "ValidationError".to_string(), "Invalid username".to_string(), details);
    /// assert!(err.details().contains_key("field"));
    /// ```
    pub fn details(&self) -> std::collections::BTreeMap<String, serde_value::Value> {
        self.details.clone()
    }
}

/// Converts an `Error` into a `std::io::Error`.
///
/// This implementation maps an `Error` to an `std::io::Error` using the
/// `InvalidData` error kind and formats the error message accordingly.
/// This allows for seamless integration with Rust's standard I/O error handling.
///
/// # Example
/// ```rust
/// use std::collections::BTreeMap;
/// use cdumay_core::{ErrorBuilder, ErrorKind};
///
/// let custom_error = ErrorBuilder::new(ErrorKind("NotFound", 404, "Not Found"), "MyNotFoundError")
///     .with_message("foo".to_string())
///     .build();
/// let io_error: std::io::Error = custom_error.into();
/// ```
impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{}", e))
    }
}

/// Implements the `Display` trait for `Error`.
///
/// This implementation formats the error as a human-readable string,
/// including its kind, class, code, and message. It provides a structured
/// error output that can be useful for logging or displaying errors in a UI.
///
/// # Format
/// ```text
/// class (code) - message
/// ```
///
/// # Example
/// ```rust
/// use cdumay_core::{ErrorBuilder, ErrorKind};
///
/// let custom_error = ErrorBuilder::new(ErrorKind("NotFound", 404, "Not Found"), "MyNotFoundError")
///     .with_message("foo".to_string())
///     .build();
/// println!("{}", custom_error);
/// ```
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{} ({}) - {}", self.class, self.code, self.message))
    }
}

/// Implements the `actix_web::ResponseError` trait for the custom `Error` type.
///
/// This allows the `Error` type to be returned directly from Actix-Web handlers,
/// automatically converting the error into an `HttpResponse` with a proper HTTP status code
/// and a serialized JSON body containing the error details.
///
/// The response is built using the `self.code` field as the HTTP status code,
/// and `self` as the JSON body. If the status code is invalid or unrecognized,
/// it defaults to `500 Internal Server Error`.
///
/// # Example (handler usage)
/// ```
/// use actix_web::{get, web, App, HttpServer};
/// use cdumay_core::{Error, Result};
///
/// #[get("/fail")]
/// async fn fail_handler() -> Result<String> {
///     Err(Error::new(400, "Custom::BadRequest".to_string(), "Invalid input".to_string(), Default::default())).into()
/// }
/// ```
///
/// # Response Format
/// The JSON response returned to the client might look like:
/// ```json
/// {
///   "code": 400,
///   "name": "Custom::BadRequest",
///   "message": "Invalid input",
///   "details": {}
/// }
/// ```
#[cfg(feature = "actix-web")]
impl actix_web::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(actix_web::http::StatusCode::from_u16(self.code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR))
            .json(self)
    }
}
