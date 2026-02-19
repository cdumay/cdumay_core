//! Tests for the `utoipa` feature: `ToSchema` implementation.
//! Only compiled when the package is built with `--features utoipa`.

#[cfg(have_utoipa)]
use cdumay_core::Error;
#[cfg(have_utoipa)]
use utoipa::ToSchema;

#[cfg(have_utoipa)]
#[test]
fn test_error_implements_to_schema() {
    fn _assert_to_schema<T: ToSchema>() {}
    _assert_to_schema::<Error>();
}

#[cfg(have_utoipa)]
#[test]
fn test_error_schema_can_be_generated() {
    let mut schemas = Vec::new();
    Error::schemas(&mut schemas);
    let _ref_or = <Error as utoipa::PartialSchema>::schema();
}

#[cfg(not(have_utoipa))]
#[test]
fn test_utoipa_skipped_without_feature() {
    // Placeholder when utoipa feature is not enabled; test file must have at least one test.
}
