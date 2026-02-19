# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.7]

### Added

- **`Error::details_ref()`** — Returns `&BTreeMap<...>` for reading details without cloning.
- **Test coverage** — Integration tests extended to reach full line coverage (100% with `--all-features`).
- **Feature-gated tests in `tests/`** — `tests/actix_web.rs` and `tests/utoipa.rs` for the `actix-web` and `utoipa` features (conditionally compiled via `build.rs` cfgs).
- **`build.rs`** — Emits `have_actix_web` and `have_utoipa` cfgs so integration tests compile only when the corresponding features are enabled.
- **Dev-dependencies** — `actix-web`, `utoipa`, and `serde_json` for running integration tests.

### Changed

- **`Error::class()` and `Error::message()`** — Now return `&str` instead of `String` to avoid allocations. Code that needs a `String` can call `.to_string()` or `.into()`.
- **`Error` `Display` impl** — Uses a single `write!` instead of `format!` + `write!` to avoid an extra allocation.
- **`ErrorConverter::store_origin`** — Takes `context` by value and reuses it instead of cloning, removing unnecessary `BTreeMap` clones.
- **Macros** — `#[inline]` added on generated getters (`code()`, `message()`, `class()`, `details()`).
- **`ErrorKind`** — `#[inline]` added on `name()`, `code()`, `description()`, and `side()`.
- **`Error`** — `#[inline]` added on `code()`, `class()`, `message()`, and `details_ref()`.
- **Test layout** — All tests now live under `tests/` (feature tests moved from `src/`).

### Fixed

- **`#[allow(unused_imports)]`** on `serde_json::json` in `error.rs` when the `utoipa` feature is enabled (used in `schema(example = ...)`).

---

[0.1.7]: https://github.com/cdumay/cdumay_core/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/cdumay/cdumay_core/releases/tag/v0.1.6
