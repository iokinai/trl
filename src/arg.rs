//! # arg
//! This module contains the `Arg` enum which represents a single argument of an attribute
//!

use crate::modifier::Modifier;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Error, Expr, ExprArray, MetaNameValue, Result, Token};

/// Struct Arg represents a single argument of an attribute
pub enum Arg {
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

impl Arg {
    /// If the parsed value is MetaNameValue, this means its the construction like: name = value.
    /// This method parses this construction into an `Arg`
    pub fn nv_to_arg(nv: &MetaNameValue) -> Result<Self> {
        match &nv
            .path
            .get_ident()
            .expect("Wrong name. Expected identifier")
            .to_string()
            .as_str()
        {
            &"includes" => Ok(Arg::Includes(Arg::brackets_to_vec(&nv.value)?)),
            &"excludes" => Ok(Arg::Excludes(Arg::brackets_to_vec(&nv.value)?)),
            &"prefix" => Ok(Arg::Prefix(Arg::ident_to_string(&nv.value)?)),
            &"name" => Ok(Arg::Name(Arg::ident_to_string(&nv.value)?)),
            &_ => Result::Err(Error::new(nv.span(), "Unknown arg name")),
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

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        let arg = if let Ok(modifier) = input.parse::<Modifier>() {
            Arg::Modifier(modifier)
        } else if input.peek(Token![pub]) {
            input.parse::<Token![pub]>()?;
            Arg::Pub
        } else if let Ok(nv) = input.parse::<MetaNameValue>() {
            Arg::nv_to_arg(&nv)?
        } else {
            return Result::Err(Error::new(input.span(), "Could not parse arg"));
        };

        Ok(arg)
    }
}
