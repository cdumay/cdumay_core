#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use serde_value::Value;
    use cdumay_core::{define_errors, define_kinds, Error};

    define_kinds! {
        NotFound = (404, "Resource Not Found"),
        Unauthorized = (401, "Unauthorized Access"),
    }

    define_errors! {
        NotFoundError = NotFound,
        UnauthorizedError = Unauthorized,
        Forbidden = (Unauthorized, 403),
    }

    #[test]
    fn test_not_found_error_defaults() {
        let err = NotFoundError::new();
        assert_eq!(err.code(), 404);
        assert_eq!(err.message(), "Resource Not Found");
        assert_eq!(err.class(), "Client::NotFound::NotFoundError");
    }

    #[test]
    fn test_forbidden_error_override_code() {
        let err = Forbidden::new();
        assert_eq!(err.code(), 403);
        assert_eq!(err.message(), "Unauthorized Access");
        assert_eq!(err.class(), "Client::Unauthorized::Forbidden");
    }

    #[test]
    fn test_with_custom_fields() {
        let mut details = BTreeMap::new();
        details.insert("reason".to_string(), Value::String("token_expired".into()));

        let err = UnauthorizedError::new()
            .with_code(498)
            .with_message("Token expired".to_string())
            .with_details(details.clone());

        assert_eq!(err.code(), 498);
        assert_eq!(err.message(), "Token expired");
        assert_eq!(err.details(), details);
    }

    #[test]
    fn test_conversion_to_core_error() {
        let err = NotFoundError::new().with_message("Page missing".to_string());
        let core: Error = err.into();

        assert_eq!(core.code(), 404);
        assert_eq!(core.message(), "Page missing");
    }
}
