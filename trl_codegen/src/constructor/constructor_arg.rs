//! # constructor_arg
//! This module contains the `ConstructorArg` enum which represents a single argument of a constructor attribute
//!

use syn::{
    Error, Expr, ExprCall, ExprLit, ExprPath, Lit, LitStr, MetaNameValue, PatLit, Result,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
};

const PUBLIC_VISIBILITY: &str = "pub";
const PRIVATE_VISIBILITY: &str = "private";

use crate::constructor::constructor_visibility::ConstructorVisibility;

/// Enum ConstructorArg represents a single argument of a constructor attribute
pub enum ConstructorArg {
    /// The constructor name. The default value is `new`
    Name(String),
    /// The constructor visibility. The default value is `ConstructorVisibility::Pub`
    Visibility(ConstructorVisibility),
}

impl ConstructorArg {
    pub fn nv_to_arg(nv: &MetaNameValue) -> Result<Self> {
        match nv
            .path
            .get_ident()
            .expect("Wrong name. Expected Identifier")
            .to_string()
            .as_str()
        {
            "name" => Ok(ConstructorArg::Name(ConstructorArg::string_from_nv_value(
                &nv.value,
            )?)),

            "visibility" => Ok(ConstructorArg::Visibility(
                ConstructorArg::visibility_from_nv_value(&nv.value)?,
            )),

            _ => Result::Err(Error::new(nv.span(), "Unknown arg name")),
        }
    }

    pub fn string_from_nv_value(e: &Expr) -> Result<String> {
        if let Expr::Path(p) = e {
            return Ok(p
                .path
                .get_ident()
                .expect("Error, expected string")
                .to_string());
        }

        return Err(Error::new(e.span(), "Error, expected string"));
    }

    pub fn visibility_parse_pub(e: String, p: &ExprPath) -> Result<ConstructorVisibility> {
        let path_text = p
            .path
            .get_ident()
            .expect("Error, expected path")
            .to_string();
        if path_text == PUBLIC_VISIBILITY {
            return Ok(ConstructorVisibility::Pub);
        } else if path_text == PRIVATE_VISIBILITY {
            return Ok(ConstructorVisibility::Private);
        } else {
            return Err(Error::new(e.span(), e));
        }
    }

    pub fn visibility_parse_pub_path(e: String, v: &String) -> Result<ConstructorVisibility> {
        if v.starts_with("pub(") && v.ends_with(")") {
            let path = &v["pub(".len()..v.len() - 1];

            return Ok(ConstructorVisibility::PubPath(path.to_string()));
        }

        Err(Error::new(e.span(), e.as_str()))
    }

    pub fn visibility_from_nv_value(e: &Expr) -> Result<ConstructorVisibility> {
        let _error_unexpected_symbol = format!(
            "Error, expected one of: \"{}\", \"{}\", \"{}\"",
            PUBLIC_VISIBILITY,
            PRIVATE_VISIBILITY,
            format!("{}(path)", PUBLIC_VISIBILITY)
        );

        if let Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) = e
        {
            if &s.value() == PUBLIC_VISIBILITY {
                Ok(ConstructorVisibility::Pub)
            } else if &s.value() == PRIVATE_VISIBILITY {
                Ok(ConstructorVisibility::Private)
            } else {
                ConstructorArg::visibility_parse_pub_path(_error_unexpected_symbol, &s.value())
            }
        } else {
            Err(Error::new(e.span(), _error_unexpected_symbol.as_str()))
        }

        // if let Expr::Path(p) = e {
        //     ConstructorArg::visibility_parse_pub(_error_unexpected_symbol, p)
        // } else if let Expr::Call(ExprCall { func, args, .. }) = e {
        //     ConstructorArg::visibility_parse_pub_path(_error_unexpected_symbol, func, args)
        // } else {
        //     Err(Error::new(e.span(), _error_unexpected_symbol.as_str()))
        // }
    }
}

impl Parse for ConstructorArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let arg = if let Ok(nv) = input.parse::<MetaNameValue>() {
            ConstructorArg::nv_to_arg(&nv)?
        } else {
            return Result::Err(Error::new(
                input.span(),
                "Could not parse arg. Expected name-value",
            ));
        };

        Ok(arg)
    }
}
