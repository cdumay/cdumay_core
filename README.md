# cdumay_core

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay_core on crates.io](https://img.shields.io/crates/v/cdumay_core)](https://crates.io/crates/cdumay_core)
[![cdumay_core on docs.rs](https://docs.rs/cdumay_core/badge.svg)](https://docs.rs/cdumay_core)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_core)

`cdumay_core` is a lightweight crate that provides custom `Error` and `Result` types
as drop-in replacements for Rust’s standard `std::result::Result` and `std::error::Error`,
with the primary goal of supporting serialization and deserialization via [`serde`].

## Motivation

Rust's standard error types do not implement `Serialize` and `Deserialize` due to
their generic and trait-based nature. This crate provides concrete, serializable
alternatives suitable for applications like:

- Returning structured errors from APIs (e.g., HTTP, RPC, WebAssembly)
- Communicating errors across process boundaries

## Features

- [`Error`] — A serializable error type with a message and optional cause.
- [`Result<T>`] — A simple alias for `crate::result::Result<T, Error>`, fully serializable.
- Full support for `serde::{Serialize, Deserialize}`.
- Optional integration with external crates via feature flags.

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

## Optional Features

- `utoipa`: Implement `utoipa::ToSchema` to `Error`
- `actix-web`: Allow to use `Result` and `Error` with actix

## Compatibility

This crate is designed for applications that require custom serialization logic.
It is **not a full replacement** for `std::result::Result` in all use cases, especially
where standard error traits are expected.

## Macros

Use the provided derive macros to define your error and error kind structs:

```rust
use cdumay_core::{define_errors, define_kinds};

define_kinds! {
    UnknownError = (500, "Unexpected error"),
    IoError = (500, "IO error")
}

define_errors! {
    Unexpected = UnknownError,
    FileRead = IoError,
    Forbidden = (IoError, 403), // kind code overwrite 500 -> 403
    FileNotFound = (IoError, 404, "File not found") // kind description overwrite
}
```
