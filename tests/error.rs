#[cfg(test)]
mod test {
    use cdumay_core::{ErrorBuilder, ErrorKind};
    
    const TEST_ERROR: ErrorKind = ErrorKind("TestError", 500, "Test error message");

    #[test]
    fn test_kind() {
        assert_eq!(TEST_ERROR.name(), "TestError");
        assert_eq!(TEST_ERROR.code(), 500);
        assert_eq!(TEST_ERROR.description(), "Test error message");
        assert_eq!(TEST_ERROR.side(), "Server");
    }
    #[test]
    fn test_error() {
        let mut details = std::collections::BTreeMap::new();
        details.insert("foo".to_string(), serde_value::to_value("foo").unwrap());

        let err = ErrorBuilder::new(TEST_ERROR, "MyError")
            .with_message("Test error".to_string())
            .with_details(details)
            .build();
        assert_eq!(format!("{}", err), "Server::TestError::MyError (500) - Test error");
    }
}
