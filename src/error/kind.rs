/// Represents a categorized error kind with associated metadata.
///
/// The `ErrorKind` struct defines a specific type of error, providing
/// a unique identifier, category, numeric code, and a message ID.
/// This allows for structured and meaningful error classification.
///
/// # Example
/// ```rust
/// use cdumay_core::ErrorKind;
/// 
/// let kind = ErrorKind("NotFound", 404, "Not Found");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorKind(
    /// A unique error identifier (name).
    pub &'static str,
    /// A numeric error code.
    pub u16,
    /// A human-readable description of the error.
    pub &'static str,
);

impl ErrorKind {
    /// Returns the name of the error.
    ///
    /// # Example
    /// ```
    /// use cdumay_core::ErrorKind;
    /// 
    /// let error = ErrorKind("NotFound", 404, "Not Found");
    /// assert_eq!(error.name(), "NotFound");
    /// ```
    pub fn name(&self) -> &'static str {
        self.0
    }

    /// Returns the numerical error code.
    ///
    /// # Example
    /// ```
    /// use cdumay_core::ErrorKind;
    ///
    /// let error = ErrorKind("NotFound", 404, "Not Found");
    /// assert_eq!(error.code(), 404);
    /// ```
    pub fn code(&self) -> u16 {
        self.1
    }

    /// Returns the description of the error.
    ///
    /// # Example
    /// ```
    /// use cdumay_core::ErrorKind;
    ///
    /// let error = ErrorKind("NotFound", 404, "Not Found");
    /// assert_eq!(error.description(), "Not Found");
    /// ```
    pub fn description(&self) -> &'static str {
        self.2
    }

    /// Determines whether the error originates from the client or the server.
    ///
    /// - Errors with codes in the range 0 to 499 are classified as **Client** errors.
    /// - Errors with codes 500 or higher are classified as **Server** errors.
    ///
    /// # Example
    /// ```
    /// use cdumay_core::ErrorKind;
    ///
    /// let client_error = ErrorKind("NotFound", 404, "Not Found");
    /// assert_eq!(client_error.side(), "Client");
    ///
    /// let server_error = ErrorKind("InternalServerError", 500, "Internal Server Error");
    /// assert_eq!(server_error.side(), "Server");
    /// ```
    pub fn side(&self) -> &'static str {
        match self.code() {
            0..=499 => "Client",
            _ => "Server",
        }
    }
}

/// Returns the default `ErrorKind`, which represents an internal server error (HTTP 500).
///
/// This is useful as a fallback error kind when no specific error type is provided.
/// It corresponds to the common "Internal Server Error" used in HTTP responses.
impl Default for ErrorKind {
    fn default() -> Self {
        ErrorKind("InternalServerError", 500, "Internal Server Error")
    }
}

