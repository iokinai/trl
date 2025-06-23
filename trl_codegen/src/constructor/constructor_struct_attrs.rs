//! constructor_struct_attrs
//! This module contains the `ConstructorStructAttrs` struct which represents the args of a constructor attribute
//!

use syn::Token;

use crate::{
    constructor::{constructor_arg::ConstructorArg, constructor_visibility::ConstructorVisibility},
    new_from_args::NewFromArgs,
};

/// Struct `ConstructorStructAttrs` represents the args of a constructor attribute
pub struct ConstructorStructAttrs {
    /// Constructor name
    pub name: String,
    /// Visibility
    pub visibility: ConstructorVisibility,
}

impl NewFromArgs<ConstructorArg> for ConstructorStructAttrs {
    fn new(args: syn::punctuated::Punctuated<ConstructorArg, Token![,]>) -> Self {
        // Constructor's default name is `new`
        let mut name = String::from("new");
        // Constructor's default visibility is `pub`
        let mut visibility = ConstructorVisibility::Pub;

        for value in args {
            match value {
                ConstructorArg::Name(n) => name = n,
                ConstructorArg::Visibility(vis) => visibility = vis,
            }
        }

        Self { name, visibility }
    }
}
