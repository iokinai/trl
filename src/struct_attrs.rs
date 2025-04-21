//! # struct_attrs
//!
//! This module contains the `StructAttrs` struct which represents struct-level attribute info

use syn::{punctuated::Punctuated, Token};

use crate::arg::Arg;
use crate::field_attrs::FieldAttrs;
use crate::modifier::Modifier;
use crate::new_from_args::NewFromArgs;

/// Represents a struct-level attribute info
#[derive(Debug, Clone)]
pub struct StructAttrs {
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

impl NewFromArgs for StructAttrs {
    fn new(punctuated: Punctuated<Arg, Token![,]>) -> StructAttrs {
        let mut includes = Vec::new();
        let mut excludes = Vec::new();
        let mut prefix = String::new();
        let mut modifier = Modifier::Ref;
        let mut include_pub = false;

        for value in punctuated {
            match value {
                Arg::Includes(i) => includes = i,
                Arg::Excludes(e) => excludes = e,
                Arg::Prefix(p) => prefix = p,
                Arg::Modifier(m) => modifier = m,
                Arg::Pub => include_pub = true,
                _ => {}
            };
        }

        StructAttrs {
            includes,
            excludes,
            prefix,
            modifier,
            include_pub,
        }
    }
}

impl Into<FieldAttrs> for StructAttrs {
    fn into(self) -> FieldAttrs {
        FieldAttrs::from_values(self.prefix, "".to_string(), self.modifier)
    }
}
