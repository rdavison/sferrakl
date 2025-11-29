# GEMINI.md

## Project Overview

This is a Rust project that provides a model for keyboard layouts. The main crate is `sferrakl-model`.

The core of the library is the `key` module, which defines the following:

*   `key::Id`: An enum representing physical keyboard keys.
*   `key::Code`: A struct representing the character produced by a key.
*   `key::Map`: A struct that maps `key::Id`s to `key::Code`s.

The library provides a way to define keyboard layouts and map physical keys to characters.

## Building and Running

### Build

To build the project, run the following command:

```bash
cargo build
```

### Test

To run the tests, run the following command:

```bash
cargo test
```

## Development Conventions

The project follows standard Rust conventions. Code is formatted with `rustfmt`.
