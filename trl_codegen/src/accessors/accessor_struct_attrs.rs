//! # struct_attrs
//!
//! This module contains the `StructAttrs` struct which represents struct-level attribute info

use syn::{Token, punctuated::Punctuated};

use crate::accessors::AccessorArg;
use crate::accessors::AccessorFieldAttrs;
use crate::modifier::Modifier;
use crate::new_from_args::NewFromArgs;

/// Represents a struct-level attribute info
#[derive(Debug, Clone)]
pub struct AccessorStructAttrs {
    /// Fields to include
    pub includes: Vec<String>,
    /// Fields to exclude
    pub excludes: Vec<String>,
    /// Prefix
    pub prefix: String,
    /// `self` modifier
    pub modifier: Modifier,
    /// Whether to include public fields or not
    pub include_pub: bool,
}

impl NewFromArgs<AccessorArg> for AccessorStructAttrs {
    fn new(punctuated: Punctuated<AccessorArg, Token![,]>) -> AccessorStructAttrs {
        let mut includes = Vec::new();
        let mut excludes = Vec::new();
        let mut prefix = String::new();
        let mut modifier = Modifier::Ref;
        let mut include_pub = false;

        for value in punctuated {
            match value {
                AccessorArg::Includes(i) => includes = i,
                AccessorArg::Excludes(e) => excludes = e,
                AccessorArg::Prefix(p) => prefix = p,
                AccessorArg::Modifier(m) => modifier = m,
                AccessorArg::Pub => include_pub = true,
                _ => {}
            };
        }

        AccessorStructAttrs {
            includes,
            excludes,
            prefix,
            modifier,
            include_pub,
        }
    }
}

impl Into<AccessorFieldAttrs> for AccessorStructAttrs {
    fn into(self) -> AccessorFieldAttrs {
        AccessorFieldAttrs::from_values(self.prefix, "".to_string(), self.modifier)
    }
}
