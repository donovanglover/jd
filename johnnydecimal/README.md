This library implements the [Johnny.Decimal Index Specification].

There are 3 main structs: `Area`, `Category`, and `Id`.

For simplicity, these struct fields are stored and compared as `String`s.

## Features

- 100% test coverage
- No unsafe code
- 100% documentation coverage

# Example

```rust
use johnnydecimal::{Area, Category, Id};

if let Ok(area) = Area::new("10-19 Area") {
    assert_eq!(area.get_name(), "Area");
}

assert!(Category::new("11 Category").is_ok());
```

# Methodology

The `struct` implementations perform validation of given `&str`s.

If validation is successful, an `Ok` is returned with the fields of that struct.

Otherwise, a friendly error message you can show to your users is returned in `Err`.

The use of `String` makes it easy to use this library in both Rust and JavaScript (through
WebAssembly/Wasm) without having to worry about custom types.

# Usage

Since I haven't published this crate to `crates.io`, you'll need to source the Git repository
directly.

```toml
[dependencies]
johnnydecimal = { git = "https://github.com/donovanglover/jd.git" }
```

# Performance

This crate uses minimal dependencies. Most functionality is achieved with Rust's standard
library, making it extremely fast to compile and use in other projects.

[Johnny.Decimal Index Specification]: https://github.com/johnnydecimal/jdcm.al__index-spec
