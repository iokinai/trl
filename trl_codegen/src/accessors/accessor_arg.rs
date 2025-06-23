//! # accessor_arg
//! This module contains the `AccessorArg` enum which represents a single argument of an attribute
//!

use crate::modifier::Modifier;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Error, Expr, ExprArray, MetaNameValue, Result, Token};

/// Enum AccessorArg represents a single argument of an accessor attribute
pub enum AccessorArg {
    /// Fields to include
    Includes(Vec<String>),
    /// Fields to exclude
    Excludes(Vec<String>),
    /// Prefix
    Prefix(String),
    /// Include public fields
    Pub,
    /// `self` modifier
    Modifier(Modifier),
    /// method (getter/setter) name
    Name(String),
}

impl AccessorArg {
    /// If the parsed value is MetaNameValue, this means its the construction like: name = value.
    /// This method parses this construction into an `Arg`
    pub fn nv_to_arg(nv: &MetaNameValue) -> Result<Self> {
        match nv
            .path
            .get_ident()
            .expect("Wrong name. Expected identifier")
            .to_string()
            .as_str()
        {
            "includes" => Ok(AccessorArg::Includes(AccessorArg::brackets_to_vec(
                &nv.value,
            )?)),
            "excludes" => Ok(AccessorArg::Excludes(AccessorArg::brackets_to_vec(
                &nv.value,
            )?)),
            "prefix" => Ok(AccessorArg::Prefix(AccessorArg::ident_to_string(
                &nv.value,
            )?)),
            "name" => Ok(AccessorArg::Name(AccessorArg::ident_to_string(&nv.value)?)),
            _ => Result::Err(Error::new(nv.span(), "Unknown arg name")),
        }
    }

    /// This method parses the `[...]` brackets expression into a vector
    pub fn brackets_to_vec(brackets: &Expr) -> Result<Vec<String>> {
        if let Expr::Array(ExprArray { elems, .. }) = brackets {
            let res = elems
                .iter()
                .map(|e| {
                    if let Expr::Path(path) = e {
                        path.path
                            .get_ident()
                            .expect("Could not parse value")
                            .to_string()
                    } else {
                        "".to_string()
                    }
                })
                .collect::<Vec<_>>();

            return Result::Ok(res);
        };

        Err(Error::new(
            brackets.span(),
            "Wrong expressing. Expected array",
        ))
    }

    /// This method parses the single identifier into string
    pub fn ident_to_string(expr: &Expr) -> Result<String> {
        if let Expr::Path(path) = expr {
            Ok(path.path.get_ident().expect("Expected ident").to_string())
        } else {
            Err(Error::new(
                expr.span(),
                "Could not parse arg value. Excpected ident",
            ))
        }
    }
}

impl Parse for AccessorArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let arg = if let Ok(modifier) = input.parse::<Modifier>() {
            AccessorArg::Modifier(modifier)
        } else if input.peek(Token![pub]) {
            input.parse::<Token![pub]>()?;
            AccessorArg::Pub
        } else if let Ok(nv) = input.parse::<MetaNameValue>() {
            AccessorArg::nv_to_arg(&nv)?
        } else {
            return Result::Err(Error::new(input.span(), "Could not parse arg"));
        };

        Ok(arg)
    }
}
