# field_access

[![Build Status](https://github.com/martinohmann/field_access/workflows/ci/badge.svg)](https://github.com/martinohmann/field_access/actions?query=workflow%3Aci)
[![crates.io](https://img.shields.io/crates/v/field_access)](https://crates.io/crates/field_access)
[![docs.rs](https://img.shields.io/docsrs/field_access)](https://docs.rs/field_access)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A library for dynamic access to struct fields with `#![no_std]` support.

Field access is enabled by the `FieldAccess` trait which can be implemented
using a derive macro by the same name.


```rust
use field_access::FieldAccess;

#[derive(FieldAccess)]
struct Foo {
    a: u8
}

let mut foo = Foo { a: 1 };

// Immutable field access.
if let Some(field) = foo.field("a") {
    assert_eq!(field.as_u8(), Some(1));
}

// Mutable field access.
if let Some(mut field) = foo.field_mut("a") {
    assert_eq!(field.replace(42u8), Some(1));
}

assert_eq!(foo.a, 42);
```

## Cargo features

- `alloc`: Provide methods to interact with types from the Rust core allocation
  and collections library including `String` and `Vec<T>`. This feature pulls
  in the `alloc` library as a dependency and is enabled by default.
- `derive`: Provide a derive macro for the `FieldAccess` trait. This feature is
  enabled by default.

## License

The source code of field_access is licensed under either of
[Apache License, Version 2.0](https://github.com/martinohmann/field_access/blob/main/LICENSE-APACHE) or
[MIT license](https://github.com/martinohmann/field_access/blob/main/LICENSE-MIT) at
your option.
