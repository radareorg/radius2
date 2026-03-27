[![crates.io](https://img.shields.io/crates/v/boolector-sys.svg)](https://crates.io/crates/boolector-sys)
[![docs.rs](https://img.shields.io/docsrs/boolector-sys)](https://docs.rs/boolector-sys)

# boolector-sys

This Rust crate provides low-level bindings for the [Boolector] SMT solver,
version 3.2.2.

[Boolector]: https://boolector.github.io/

## Installation

### Using shared `boolector` library

Compile `boolector` as a shared library and install it.  Then add this crate
to your `Cargo.toml`:

```toml
[dependencies]
boolector-sys = "0.7.2"
```

### Using vendored static `boolector` library

This is possible on UNIX-like targets only.  Add this crate to your `Cargo.toml`
with the `vendor-lgl` feature enabled:

```toml
[dependencies]
boolector-sys = { version = "0.7.2", features = ["vendor-lgl"] }
```

Enabling `vendor-lgl` will automatically build a static `boolector` library and
link against it.  Currently this uses the Lingeling SAT solver.

In order for the build to succeed, you'll need to install some tools on your
build host; for a Debian-based distribution `build-essential`, `cmake`, `curl`,
and `git` should be sufficient.

## License

This crate is licensed under the [MIT license].

[MIT license]: LICENSE
