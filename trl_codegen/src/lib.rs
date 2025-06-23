//! # TRL - type reflection library
//!
//! This library provides auto generation of some common methods based on Rust macros
//!
//! All future examples will use this test struct:
//! ```rust,ignore
//! struct User {
//!     id: u32,
//!     name: String,
//!     email: String,
//!
//!     // public fields are ignored by default
//!     pub phone_number: u64,
//!}
//! ```
//!
//! This library contains 2 types of macros:
//! - Struct level - macros applied to a struct
//! - Field level - macros applied to a single field
//!
//! Both of them have addictional arguments
//!
//! ### Struct level macros
//! Struct level macros are: `getters` and `setters`. They generates getters/setters for all the fields of the struct
//! They have common arguments:
//! - include=[...]
//! - exclude=[...]
//! - pub
//! - prefix=...
//! ###
//! - include=\[...\] - generate getter/setters only for the listed fields.
//! For example:
//! ```rust,ignore
//! #[derive(trl)]
//! #[getters(include=[name, email])]
//! #[setters(include=[name, email])]
//! struct Test {/* ... */}
//! ```
//!
//! Would generate getters/setters for `b` and `c` fields.
//!
//! - exclude=\[...\] - generate getters/setters for all fields except the listed.
//! For example
//!
//! ```rust,ignore
//! #[derive(trl)]
//! #[getters(exclude=[a, b])]
//! #[setters(exclude=[a, b])]
//! struct Test { /* ... */ }
//! ```
//!
//! Would generate getters/setters only for the `c` field.
//!
//! - pub - include public fields.
//! By default public fields are ignored, but you can specify the `pub` argument to generate getters/setters for them too
//!
//! - prefix=... - generates getters/setters with specified prefix.
//! For example
//! ```rust,ignore
//! #[derive(trl)]
//! #[getters(prefix=get_)]
//! #[setters(prefix=set_)]
//! struct Test { /* ... */ }
//! ```
//! Would generate getters:
//! - `get_id()`, `get_name()`
//! and setters:
//! - `set_id()`, `set_name()`
//!
//! Default value for getters is empty string, and for setters is `set_`
//!
//! ### Field level macros
//! Field level macros are `get` and `set`. They generates a getter/setter for a single field.
//! They have common arguments:
//! - name = ... - generate a getter/setter with the specified name
//! - prefix = ... - generate a getter/setter with the specified prefix

mod accessors;
mod attribute_parser;
mod constructor;
mod helpers;
mod modifier;
mod new_from_args;

use attribute_parser::generate_impl_for_struct;
use syn::{ItemStruct, parse_macro_input};

/// Default macro which is required by all others
/// ### Attributes:
/// - `#[get]` - generate a getter for a single field
/// - `#[set]` - generate a setter for a single field
///
/// ### Common arguments
/// - name = ... - generate a getter/setter with the specified name
/// - prefix = ... - generate a getter/setter with the specified prefix
///
/// ### Special agruments for `get`
/// - ref / mut ref / move - specify the self modifier
#[proc_macro_derive(trl, attributes(get, set))]
pub fn trl_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    generate_impl_for_struct(&input)
}

/// Generate getters for a struct fields
/// ### Common arguments
/// - include=\[...\] - generate getter/setters only for the listed fields.
/// For example:
/// ```rust,ignore
/// #[derive(trl)]
/// #[getters(include=[name, email])]
/// #[setters(include=[name, email])]
/// struct Test {/* ... */}
/// ```
///
/// Would generate getters/setters for `b` and `c` fields.
///
/// - exclude=\[...\] - generate getters/setters for all fields except the listed.
/// For example
///
/// ```rust,ignore
/// #[derive(trl)]
/// #[getters(exclude=[a, b])]
/// #[setters(exclude=[a, b])]
/// struct Test { /* ... */ }
/// ```
///
/// Would generate getters/setters only for the `c` field.
///
/// - pub - include public fields.
/// By default public fields are ignored, but you can specify the `pub` argument to generate getters/setters for them too
///
/// - prefix=... - generates getters/setters with specified prefix.
/// For example
/// ```rust,ignore
/// #[derive(trl)]
/// #[getters(prefix=get_)]
/// #[setters(prefix=set_)]
/// struct Test { /* ... */ }
/// ```
/// Would generate getters:
/// - `get_id()`, `get_name()`
/// and setters:
/// - `set_id()`, `set_name()`
///
/// Default value for getters is empty string, and for setters is `set_`
///
/// ### Special agruments
/// - ref / mut ref / move - specify the self modifier
///
/// For example:
/// ```,rust,ignore
/// #[derive(trl)]
/// #[getters(mut ref)]
/// struct Test { /* ... */ }
/// ```
/// Would generate getters where `self` is taken as `&mut self`
///
#[proc_macro_attribute]
pub fn getters(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

/// Generate setters for a struct fields
/// /// ### Common arguments
/// - include=\[...\] - generate getter/setters only for the listed fields.
/// For example:
/// ```rust,ignore
/// #[derive(trl)]
/// #[getters(include=[name, email])]
/// #[setters(include=[name, email])]
/// struct Test {/* ... */}
/// ```
///
/// Would generate getters/setters for `b` and `c` fields.
///
/// - exclude=\[...\] - generate getters/setters for all fields except the listed.
/// For example
///
/// ```rust,ignore
/// #[derive(trl)]
/// #[getters(exclude=[a, b])]
/// #[setters(exclude=[a, b])]
/// struct Test { /* ... */ }
/// ```
///
/// Would generate getters/setters only for the `c` field.
///
/// - pub - include public fields.
/// By default public fields are ignored, but you can specify the `pub` argument to generate getters/setters for them too
///
/// - prefix=... - generates getters/setters with specified prefix.
/// For example
/// ```rust,ignore
/// #[derive(trl)]
/// #[getters(prefix=get_)]
/// #[setters(prefix=set_)]
/// struct Test { /* ... */ }
/// ```
/// Would generate getters:
/// - `get_id()`, `get_name()`
/// and setters:
/// - `set_id()`, `set_name()`
///
/// Default value for getters is empty string, and for setters is `set_`
///
/// ### Special agruments
/// - ref / mut ref / move - specify the self modifier
///
/// For example:
/// ```rust,ignore
/// #[derive(trl)]
/// #[getters(mut ref)]
/// struct Test { /* ... */ }
/// ```
/// Would generate getters where `self` is taken as `&mut self`
///
#[proc_macro_attribute]
pub fn setters(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

/// Generate default constructor for a struct
/// ### Common arguments
/// - name=... - generate a constructor with specified name.
/// Default name is `new`
/// For example:
/// ```rust,ignore
/// #[derive(trl)]
/// #[constructor(name = new_test)]
/// struct Test {/* ... */}
/// ```
///
/// Would generate constructor with name `new_test`.
///
/// - visibility=... - generate a constructor with specified visibility modifier
///
/// Possible modifiers are `pub``, `pub(path)` and `private`
/// For example
///
/// ```rust,ignore
/// #[derive(trl)]
/// #[constructor(visibility = pub(crate))]
/// struct Test { /* ... */ }
/// ```
///
/// Would generate constructor with `pub(crate)` visibility specifier.
///
#[proc_macro_attribute]
pub fn constructor(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
