//! Provides structs to manipulate result
/// A simplified `Result` type for application-level operations.
///
/// This enum represents either success (`Ok`) with a value of type `T`,
/// or failure (`Err`) with a custom application error type (`cdumay_core::error::Error`).
///
/// It is similar to `std::result::Result`, but specialized for your crate's error handling.
///
/// # Example
/// ```
/// use cdumay_core::Result;
///
/// fn double(x: i32) -> Result<i32> {
///     Result::Ok(x * 2)
/// }
/// ```
#[derive(Debug)]
pub enum Result<T> {
    /// Contains the success value.
    Ok(T),
    /// Contains the application error.
    Err(crate::Error),
}

impl<T> Result<T> {
    /// Maps a `Result<T>` to `Result<U>` by applying a function to a contained `Ok` value,
    /// leaving an `Err` untouched.
    ///
    /// # Arguments
    /// * `f` - A function to apply to the `Ok` value.
    ///
    /// # Returns
    /// A new `Result<U>` with the result of the function if `self` is `Ok`,
    /// or the original error if `self` is `Err`.
    ///
    /// # Example
    /// ```
    /// use cdumay_core::Result;
    ///
    /// let result: Result<i32> = Result::Ok(2);
    /// let mapped = result.map(|x| x * 10);
    /// assert!(matches!(mapped, Result::Ok(20)));
    /// ```
    pub fn map<U, F>(self, f: F) -> Result<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Result::Ok(t) => Result::Ok(f(t)),
            Result::Err(e) => Result::Err(e),
        }
    }
}

impl<S> From<std::result::Result<S, crate::Error>> for Result<S> {
    fn from(res: std::result::Result<S, crate::Error>) -> Self {
        match res {
            Ok(data) => Result::Ok(data),
            Err(err) => Result::Err(err),
        }
    }
}

#[cfg(feature = "actix-web")]
use actix_web::ResponseError;

#[cfg(feature = "actix-web")]
impl<T> actix_web::Responder for Result<T>
where
    T: serde::Serialize,
{
    type Body = actix_web::body::BoxBody;

    /// Converts the `Result<T>` into an `HttpResponse` for Actix-Web.
    ///
    /// If `Ok`, the inner value is converted using its own `Responder` implementation.
    /// If `Err`, the error is converted using its `ResponseError::error_response()` implementation.
    ///
    /// # Example (Actix handler)
    /// ```
    /// use actix_web::{web, App, HttpServer, HttpResponse};
    /// use cdumay_core::Result;
    ///
    /// async fn handler() -> Result<String> {
    ///     Ok("Hello from handler!".to_string()).into()
    /// }
    /// ```
    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match self {
            Result::Ok(val) => actix_web::HttpResponse::Ok().json(val),
            Result::Err(e) => e.error_response(),
        }
    }
}
