/// Defines a set of constant `ErrorKind` values for reuse across the application.
///
/// This macro generates `pub const` instances of `ErrorKind` using a simple and concise syntax.
///
/// # Syntax
///
/// ```rust
/// use cdumay_core::define_kinds;
///
/// define_kinds! {
///     NotFound = (404, "Resource Not Found"),
///     Unauthorized = (401, "Unauthorized Access"),
/// }
/// ```
///
/// This expands to:
///
/// ```rust
/// use cdumay_core::ErrorKind;
///
/// pub const NotFound: ErrorKind = ErrorKind("NotFound", 404, "Resource Not Found");
/// pub const Unauthorized: ErrorKind = ErrorKind("Unauthorized", 401, "Unauthorized Access");
/// ```
///
/// These constants can be used directly in your code or passed into higher-level error builders.
#[macro_export]
macro_rules! define_kinds {
    (
        $(
            $ident:ident = ($code:expr, $description:expr)
        ),* $(,)?
    ) => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $ident: cdumay_core::ErrorKind = cdumay_core::ErrorKind(stringify!($ident), $code, $description);
        )*
    };
}

/// Defines structured error types tied to specific `ErrorKind` constants.
///
/// This macro generates concrete error structs with built-in support for:
/// - Default and customizable HTTP-style status codes
/// - Human-readable error messages
/// - Structured error details (as a serializable map)
/// - Conversion to a unified `Error` type
///
/// # Syntax
///
/// ```rust
/// use cdumay_core::{define_errors, define_kinds};
///
/// define_kinds! {
///     NotFound = (404, "Resource Not Found"),
///     Unauthorized = (401, "Unauthorized Access"),
/// }
///
/// define_errors! {
///     NotFoundError = NotFound,
///     UnauthorizedError = Unauthorized,
/// }
/// ```
///
/// This expands to:
/// - A `struct` for each error type (e.g., `NotFoundError`)
/// - Methods to configure error code, message, and details
/// - Implementations of `std::error::Error`, `Display`, and `From<T> for Error`
///
/// The generated errors are intended for use in APIs or services where structured,
/// serializable errors are preferred.
///
/// > **Note**: Requires a corresponding constant to be defined using [`define_kinds!`].
#[macro_export]
macro_rules! define_errors {
    (
        $(
            $name:ident = $kind:ident
        ),* $(,)?
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                code: Option<u16>,
                message: Option<String>,
                details: Option<std::collections::BTreeMap<String, serde_value::Value>>,
            }
            
            impl $name {
                pub fn new() -> Self {
                    Self {
                        code: None,
                        message: None,
                        details: None,
                    }
                }
                pub const kind: cdumay_core::ErrorKind = $kind;
                
                pub fn code(&self) -> u16 {
                    self.code.unwrap_or($name::kind.code())
                }
                pub fn with_code(mut self, code: u16) -> Self {
                    self.code = Some(code);
                    self
                }
                pub fn message(&self) -> String {
                    self.message.clone().unwrap_or($name::kind.description().to_string())
                }
                pub fn with_message(mut self, message: String) -> Self {
                    self.message = Some(message);
                    self
                }
                pub fn details(&self) -> std::collections::BTreeMap<String, serde_value::Value> {
                    self.details.clone().unwrap_or_default()
                }
                pub fn with_details(mut self, details: std::collections::BTreeMap<String, serde_value::Value>) -> Self {
                    self.details = Some(details);
                    self
                }
                pub fn class(&self) -> String {
                    format!("{}{}{}", Self::kind.side(), Self::kind.name(), stringify!($name))
                }
            }
            
            impl std::error::Error for $name {}
        
            impl From<$name> for cdumay_core::Error {
                fn from(err: $name) -> cdumay_core::Error {
                    cdumay_core::ErrorBuilder::new($name::kind, stringify!($name))
                        .with_code(err.code())
                        .with_message(err.message())
                        .with_details(err.details())
                        .build()
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{} ({}): {}", self.class(), self.code(), self.message())
                }
            }
        )*
    };
}
