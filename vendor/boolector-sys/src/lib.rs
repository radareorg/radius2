//! Low-level bindings for the [Boolector] SMT solver.
//!
//! Please see the Boolector [C API documentation] for function descriptions.
//!
//! [Boolector]: https://boolector.github.io/
//! [C API documentation]: https://boolector.github.io/docs/cboolector.html

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

include!("../src-generated/bindings.rs");
include!("../src-generated/options.rs");
