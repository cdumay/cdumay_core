// Emit cfgs so integration tests in tests/ can be conditionally compiled based on package features.
fn main() {
    println!("cargo:rustc-check-cfg=cfg(have_actix_web)");
    println!("cargo:rustc-check-cfg=cfg(have_utoipa)");
    if std::env::var("CARGO_FEATURE_ACTIX_WEB").is_ok() {
        println!("cargo:rustc-cfg=have_actix_web");
    }
    if std::env::var("CARGO_FEATURE_UTOIPA").is_ok() {
        println!("cargo:rustc-cfg=have_utoipa");
    }
}
