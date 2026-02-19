#[cfg(test)]
mod test {
    use cdumay_core::{Error, ErrorBuilder, ErrorKind};
    use std::collections::BTreeMap;

    const TEST_ERROR: ErrorKind = ErrorKind("TestError", 500, "Test error message");

    #[test]
    fn test_kind() {
        assert_eq!(TEST_ERROR.name(), "TestError");
        assert_eq!(TEST_ERROR.code(), 500);
        assert_eq!(TEST_ERROR.description(), "Test error message");
        assert_eq!(TEST_ERROR.side(), "Server");
    }

    #[test]
    fn test_kind_client_side() {
        let client = ErrorKind("NotFound", 404, "Not Found");
        assert_eq!(client.side(), "Client");
    }

    #[test]
    fn test_kind_default() {
        let default = ErrorKind::default();
        assert_eq!(default.name(), "InternalServerError");
        assert_eq!(default.code(), 500);
        assert_eq!(default.description(), "Internal Server Error");
        assert_eq!(default.side(), "Server");
    }

    #[test]
    fn test_error_builder_full() {
        let mut details = BTreeMap::new();
        details.insert("foo".to_string(), serde_value::to_value("foo").unwrap());

        let err = ErrorBuilder::new(TEST_ERROR, "MyError")
            .with_code(503)
            .with_message("Test error".to_string())
            .with_details(details.clone())
            .build();
        assert_eq!(err.code(), 503);
        assert_eq!(err.class(), "Server::TestError::MyError");
        assert_eq!(err.message(), "Test error");
        assert_eq!(err.details_ref().get("foo").unwrap(), &serde_value::to_value("foo").unwrap());
        assert_eq!(err.details(), details);
        assert_eq!(format!("{}", err), "Server::TestError::MyError (503) - Test error");
    }

    #[test]
    fn test_error_builder_defaults() {
        let err = ErrorBuilder::new(TEST_ERROR, "Minimal").build();
        assert_eq!(err.code(), 500);
        assert_eq!(err.class(), "Server::TestError::Minimal");
        assert_eq!(err.message(), "Test error message");
        assert!(err.details_ref().is_empty());
        assert!(err.details().is_empty());
    }

    #[test]
    fn test_error_builder_default_trait() {
        let err = ErrorBuilder::default().build();
        assert_eq!(err.code(), 500);
        assert_eq!(err.class(), "Server::InternalServerError::UnknownError");
        assert_eq!(err.message(), "Internal Server Error");
        assert_eq!(format!("{}", err), "Server::InternalServerError::UnknownError (500) - Internal Server Error");
    }

    #[test]
    fn test_error_direct_new() {
        let mut details = BTreeMap::new();
        details.insert("key".to_string(), serde_value::Value::String("value".to_string()));
        let err = Error::new(
            400,
            "ValidationError".to_string(),
            "Invalid input".to_string(),
            details.clone(),
        );
        assert_eq!(err.code(), 400);
        assert_eq!(err.class(), "ValidationError");
        assert_eq!(err.message(), "Invalid input");
        assert_eq!(err.details_ref().get("key").unwrap(), &serde_value::Value::String("value".to_string()));
        assert_eq!(err.details(), details);
        assert_eq!(format!("{}", err), "ValidationError (400) - Invalid input");
    }

    #[test]
    fn test_error_into_io_error() {
        let err = ErrorBuilder::new(TEST_ERROR, "IoTest")
            .with_message("file not found".to_string())
            .build();
        let io_err: std::io::Error = err.into();
        assert_eq!(io_err.kind(), std::io::ErrorKind::InvalidData);
        assert!(io_err.to_string().contains("file not found"));
    }
}
