//! # field_attrs
//!
//! This module contains the `FieldAttrs` struct, which holds information about a field
//! for which a method is going to be generated.

use syn::{Token, punctuated::Punctuated};

use crate::{accessors::AccessorArg, modifier::Modifier, new_from_args::NewFromArgs};

/// Information about field for which a method is going to be generated
#[derive(Debug)]
pub struct AccessorFieldAttrs {
    /// Method prefix
    pub prefix: String,
    /// Method name
    pub name: String,
    /// Method `self` modifier
    pub modifier: Modifier,
}

impl AccessorFieldAttrs {
    pub fn from_values(prefix: String, name: String, modifier: Modifier) -> AccessorFieldAttrs {
        AccessorFieldAttrs {
            prefix,
            name,
            modifier,
        }
    }
}

impl NewFromArgs<AccessorArg> for AccessorFieldAttrs {
    fn new(args: Punctuated<AccessorArg, Token![,]>) -> AccessorFieldAttrs {
        let mut prefix = String::new();
        let mut name = String::new();
        let mut modifier = Modifier::Ref;

        for arg in args {
            match arg {
                AccessorArg::Prefix(p) => prefix = p,
                AccessorArg::Name(n) => name = n,
                AccessorArg::Modifier(m) => modifier = m,
                _ => {}
            }
        }

        AccessorFieldAttrs {
            prefix,
            name,
            modifier,
        }
    }
}
