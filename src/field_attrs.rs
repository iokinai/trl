use syn::{punctuated::Punctuated, Meta, Token};

use crate::{helpers::parse_expr_str, modifier::Modifier, new_from_args::NewFromArgs};

#[derive(Debug)]
pub struct FieldAttrs {
    pub prefix: String,
    pub name: String,
    pub modifier: Modifier,
}

impl FieldAttrs {
    pub(crate) fn from_values(prefix: String, name: String, modifier: Modifier) -> FieldAttrs {
        FieldAttrs {
            prefix,
            name,
            modifier,
        }
    }
}

impl NewFromArgs for FieldAttrs {
    fn new(args: Punctuated<Meta, Token![,]>) -> FieldAttrs {
        let mut prefix = String::new();
        let mut name = String::new();
        let mut modifier = Modifier::Ref;

        for meta in args {
            match meta {
                Meta::NameValue(nv) => match nv.path.get_ident().unwrap().to_string().as_str() {
                    "prefix" => parse_expr_str(&nv.value, &mut prefix),
                    "name" => parse_expr_str(&nv.value, &mut name),
                    _ => {}
                },
                Meta::Path(p) => match p.get_ident().unwrap().to_string().as_str() {
                    "borrow" => modifier = Modifier::Borrow,
                    "mut_ref" => modifier = Modifier::MutRef,
                    _ => {}
                },
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
