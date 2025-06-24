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
//!
//! ### Constructor
//! Constructor is a struct-level macro which generates the default constructor:
//!
//! ```rust,ignore
//! #[derive(trl)]
//! #[constructor]
//! struct Test {/* ... */}
//! ```
//!
//! Would genetate:
//! ```rust,ignore
//! pub fn new(id: u32, name: String, email: String, phone_number: u64) -> Self {
//!     Self {
//!         id,
//!         name,
//!         email,
//!         phone_number,
//!     }
//! }
//! ```
//!
//! - name - generate constructor with specified name
//! - visibility - generate constructor with specified visibility modifier
//!
//! Possible visibilities (must be specified as string literals)
//! - `"pub"` - public visibility
//! - `"pub(path)"` - restricted public visibility (e.g. "pub(crate)", "pub(super)", "pub(in some::module)")
//! - `"private"` - private visibility (not actually a Rust keyword, but used here for convenience)
//!
//! For example this:
//!
//! ```rust,ignore
//! #[derive(trl)]
//! #[constructor(name = new_test, visibility = "pub(crate)")]
//! struct Test { /* ... */ }
//! ```
//!
//! Would generate:
//!
//! ```rust,ignore
//! pub(crate) fn new_test(id: u32, name: String, email: String, phone_number: u64) -> Self {
//!     Self {
//!         id,
//!         name,
//!         email,
//!         phone_number,
//!     }
//! }
//! ```
//!

pub use trl_codegen::*;

pub mod prelude {
    //! Module `prelude` reexports all commonly used attributes.
    //!
    //! It's recommended to always import it to simplify usage:
    //! ```rust
    //! use trl::prelude::*;
    //! ```
    //!
    //! ⚠️ **Important:** All attributes must be *directly imported* and
    //! used by their plain names. For example, use:
    //! ```rust
    //! #[constructor]
    //! ```
    //! and **not**
    //! ```rust
    //! #[trl::constructor] // ← This will NOT work
    //! ```
    //!
    //! This limitation exists due to the current implementation.
    pub use crate::constructor;
    pub use crate::getters;
    pub use crate::setters;
    pub use crate::trl;
}

#[cfg(test)]
mod tests {
    use trl_codegen::{constructor, getters, setters, trl};

    #[derive(Default, trl)]
    #[getters]
    struct OnlyGettersUser {
        id: u64,
        name: String,
        email: String,
        pub phone_number: u64,
    }

    #[test]
    fn getters_test() {
        let user = OnlyGettersUser::default();

        assert!(*user.id() == 0);
        assert!(*user.name() == String::from(""));
        assert!(*user.email() == String::from(""));
    }

    #[derive(Default, trl)]
    #[getters(pub)]
    struct GettersIncludePubUser {
        id: u64,
        name: String,
        email: String,
        pub phone_number: u64,
    }

    #[test]
    fn include_pub_test() {
        let user = GettersIncludePubUser::default();

        assert!(*user.phone_number() == 0)
    }

    #[derive(Default, trl)]
    struct GetSetFieldUser {
        #[set]
        id: u64,
        #[get(name = get_name)]
        name: String,
        #[get]
        email: String,
        pub phone_number: u64,
    }

    #[test]
    fn get_one_test() {
        let mut user = GetSetFieldUser::default();

        assert!(user.id == 0);
        user.set_id(2);
        assert!(*user.get_name() == String::from(""));
        assert!(*user.email() == String::from(""));
        assert!(user.id == 2);
    }

    #[derive(Default, trl)]
    #[setters]
    struct SettersUser {
        id: u64,
        name: String,
        email: String,
        pub phone_number: u64,
    }

    #[test]
    fn setters_test() {
        let mut user = SettersUser::default();

        assert!(user.name == "");
        user.set_name(String::from("John"));
        assert!(user.name == "John");
    }

    #[derive(Default, trl)]
    #[constructor]
    struct ConstructorUser {
        id: u64,
        name: String,
        email: String,
        pub phone_number: u64,
    }

    #[test]
    fn constructor_test() {
        let user = ConstructorUser::new(0, String::new(), String::new(), 3);
    }
}
