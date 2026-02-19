//! Tests for the `actix-web` feature: `ResponseError` implementation.
//! Only compiled when the package is built with `--features actix-web`.

#[cfg(have_actix_web)]
use actix_web::ResponseError;
#[cfg(have_actix_web)]
use cdumay_core::{Error, ErrorBuilder, ErrorKind};
#[cfg(have_actix_web)]
use std::collections::BTreeMap;

#[cfg(have_actix_web)]
#[test]
fn test_response_error_builds_http_response() {
    let err = Error::new(
        404,
        "Client::NotFound::NotFound".to_string(),
        "Resource not found".to_string(),
        BTreeMap::new(),
    );
    let response = err.error_response();
    assert_eq!(response.status().as_u16(), 404);
}

#[cfg(have_actix_web)]
#[test]
fn test_response_error_invalid_status_defaults_to_500() {
    let err = Error::new(
        9999,
        "Test::Invalid::Code".to_string(),
        "Invalid".to_string(),
        BTreeMap::new(),
    );
    let response = err.error_response();
    assert_eq!(response.status().as_u16(), 500);
}

#[cfg(have_actix_web)]
#[test]
fn test_response_error_returns_json_content_type() {
    let mut details = BTreeMap::new();
    details.insert(
        "key".to_string(),
        serde_value::Value::String("value".to_string()),
    );
    let err = ErrorBuilder::new(
        ErrorKind("ValidationError", 400, "Bad request"),
        "InvalidInput",
    )
    .with_code(400)
    .with_message("Invalid input".to_string())
    .with_details(details)
    .build();
    let response = err.error_response();
    assert_eq!(response.status().as_u16(), 400);
    let ct: Option<&str> = response
        .headers()
        .get("content-type")
        .and_then(|v: &actix_web::http::header::HeaderValue| v.to_str().ok());
    assert!(
        ct.map_or(false, |s: &str| s.starts_with("application/json")),
        "expected JSON content-type, got {:?}",
        ct
    );
}

#[cfg(not(have_actix_web))]
#[test]
fn test_actix_web_skipped_without_feature() {
    // Placeholder when actix-web feature is not enabled; test file must have at least one test.
}
