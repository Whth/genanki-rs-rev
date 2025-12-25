# Tests

This directory contains all tests for the genanki-rs crate, organized by functionality.

## Structure

- `note_tests.rs` - Tests for the Note struct and related functionality
- `model_tests.rs` - Tests for the Model struct and related functionality  
- `builtin_models_tests.rs` - Tests for the built-in model functions
- `lib_tests.rs` - Integration tests and tests for library functionality
- `mod.rs` - Module declarations for organizing tests

## Running Tests

Run all tests with:
```bash
cargo test
```

Run specific test modules with:
```bash
cargo test note_tests
cargo test model_tests
```

## Test Organization

The tests were moved from inline `#[cfg(test)]` modules in the source files to separate files in this directory to:
- Improve organization and maintainability
- Make it easier to find and run specific tests
- Reduce compilation times for non-test builds
- Follow Rust best practices for test organization