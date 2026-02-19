# cdumay_core

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay_core on crates.io](https://img.shields.io/crates/v/cdumay_core)](https://crates.io/crates/cdumay_core)
[![cdumay_core on docs.rs](https://docs.rs/cdumay_core/badge.svg)](https://docs.rs/cdumay_core)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_core)

A lightweight Rust crate that provides custom `Error` and `Result` types as drop-in replacements for `std::result::Result` and `std::error::Error`, with full serialization support via [serde](https://serde.rs).

## Motivation

Rust’s standard error types do not implement `Serialize` and `Deserialize`. This crate provides concrete, serializable alternatives for:

- Returning structured errors from APIs (HTTP, RPC, WebAssembly)
- Communicating errors across process boundaries

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cdumay_core = "0.1"
```

Optional features:

```toml
cdumay_core = { version = "0.1", features = ["utoipa", "actix-web"] }
```

## Features (crate)

- **`Error`** — Serializable error with code, class, message, and optional details map.
- **`Result<T>`** — Type alias for `Result<T, Error>`, serializable.
- **`ErrorKind`** — Categorized kind (name, code, description); client/server side.
- **`ErrorBuilder`** — Fluent builder for `Error`.
- **`ErrorConverter`** — Trait to convert custom errors into `Error` with context.
- **Macros** — `define_kinds!` and `define_errors!` for structured error definitions.

## Optional features (flags)

| Feature     | Description                                      |
|------------|--------------------------------------------------|
| `utoipa`   | Implements `utoipa::ToSchema` for `Error`.       |
| `actix-web`| Implements `actix_web::ResponseError` for `Error`. |

Enable in `Cargo.toml`:

```toml
cdumay_core = { version = "0.1", features = ["utoipa", "actix-web"] }
```

## Example

```rust
use cdumay_core::{ErrorBuilder, Result};

fn do_work() -> Result<i32> {
    Err(
        ErrorBuilder::default()
            .with_message("Something went wrong".to_string())
            .build()
    ).into()
}
```

## Macros

Use `define_kinds!` and `define_errors!` to define reusable error kinds and concrete error types:

```rust
use cdumay_core::{define_errors, define_kinds};

define_kinds! {
    UnknownError = (500, "Unexpected error"),
    IoError = (500, "IO error")
}

define_errors! {
    Unexpected = UnknownError,
    FileRead = IoError,
    Forbidden = (IoError, 403),              // override code 500 → 403
    FileNotFound = (IoError, 404, "File not found")  // override description
}
```

Generated types support `Display`, `std::error::Error`, and `From<T> for Error`.

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with all optional features (actix-web, utoipa):

```bash
cargo test --all-features
```

Tests live under the `tests/` directory; feature-gated tests run only when the corresponding feature is enabled.

## Compatibility

This crate is intended for applications that need serializable errors. It is not a full replacement for `std::result::Result` in every context, especially where standard error traits or other error types are required.

## Documentation

- [API docs on docs.rs](https://docs.rs/cdumay_core)
- [Repository](https://github.com/cdumay/cdumay_core)

## License

BSD-3-Clause — see [LICENSE](./LICENSE).
