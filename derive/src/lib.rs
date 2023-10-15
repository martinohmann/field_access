//! This crate provides `field_access`'s derive macro.
//!
//! ```no_compile
//! #[derive(FieldAccess)]
//! ```
//!
//! Please refer to the documentation of the [`field_access`](https://docs.rs/field_access) crate
//! for how to set this up.
extern crate proc_macro;

mod expand;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FieldAccess)]
pub fn derive_field_access(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
