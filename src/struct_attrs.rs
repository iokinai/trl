use syn::{punctuated::Punctuated, Meta, Token};

use crate::field_attrs::FieldAttrs;
use crate::helpers::{parse_expr_array, parse_expr_str};
use crate::modifier::Modifier;
use crate::new_from_args::NewFromArgs;

#[derive(Debug, Clone)]
pub struct StructAttrs {
    pub includes: Vec<String>,
    pub excludes: Vec<String>,
    pub prefix: String,
    pub modifier: Modifier,
    pub include_pub: bool,
}

impl NewFromArgs for StructAttrs {
    fn new(punctuated: Punctuated<Meta, Token![,]>) -> StructAttrs {
        let mut includes = Vec::new();
        let mut excludes = Vec::new();
        let mut prefix = String::new();
        let mut modifier = Modifier::Ref;
        let mut include_pub = false;

        for value in punctuated {
            match value {
                Meta::NameValue(nv) => match nv.path.get_ident().unwrap().to_string().as_str() {
                    "includes" => parse_expr_array(&nv.value, &mut includes),
                    "excludes" => parse_expr_array(&nv.value, &mut excludes),
                    "prefix" => parse_expr_str(&nv.value, &mut prefix),
                    _ => {}
                },
                Meta::Path(p) => match p.get_ident().unwrap().to_string().as_str() {
                    "borrow" => modifier = Modifier::Borrow,
                    "mut_ref" => modifier = Modifier::MutRef,
                    "include_pub" => include_pub = true,
                    _ => {}
                },
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
