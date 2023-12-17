`jd` is a CLI for [Johnny.Decimal] systems.

It consists of a public library that implements the `System` struct.

# Example

```rust
use jd::System;

assert!(System::new("../test_systems/simple").is_ok());
```

# Usage

Since I haven't published this crate to `crates.io`, you'll need to source the Git repository
directly.

```toml
[dependencies]
jd = { git = "https://github.com/donovanglover/jd.git" }
```

# Performance

`jd` works by assuming that the `Index` is the ultimate source of truth, making it possible to
perform operations without querying the filesystem every time beforehand.

[Johnny.Decimal]: https://johnnydecimal.com/
