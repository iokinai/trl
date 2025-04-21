//! # field_attrs
//!
//! This module contains the `FieldAttrs` struct, which holds information about a field
//! for which a method is going to be generated.

use syn::{punctuated::Punctuated, Token};

use crate::{arg::Arg, modifier::Modifier, new_from_args::NewFromArgs};

/// Information about field for which a method is going to be generated
#[derive(Debug)]
pub struct FieldAttrs {
    /// Method prefix
    pub prefix: String,
    /// Method name
    pub name: String,
    /// Method `self` modifier
    pub modifier: Modifier,
}

impl FieldAttrs {
    pub fn from_values(prefix: String, name: String, modifier: Modifier) -> FieldAttrs {
        FieldAttrs {
            prefix,
            name,
            modifier,
        }
    }
}

impl NewFromArgs for FieldAttrs {
    fn new(args: Punctuated<Arg, Token![,]>) -> FieldAttrs {
        let mut prefix = String::new();
        let mut name = String::new();
        let mut modifier = Modifier::Ref;

        for arg in args {
            match arg {
                Arg::Prefix(p) => prefix = p,
                Arg::Name(n) => name = n,
                Arg::Modifier(m) => modifier = m,
                _ => {}
            }
        }

        FieldAttrs {
            prefix,
            name,
            modifier,
        }
    }
}
