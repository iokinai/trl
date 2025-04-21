//! # modifier
//!
//! This module contains the `Modifier` enum which represents the `self` modifier

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Token,
};

/// Represents the `self` modifier
#[derive(Debug, Clone)]
pub enum Modifier {
    /// fn get(self)
    Move,
    // fn get(&self)
    Ref,
    // fn get(&mut self)
    MutRef,
}

impl Parse for Modifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![mut]) && input.peek2(Token![ref]) {
            input.parse::<Token![mut]>()?;
            input.parse::<Token![ref]>()?;
            Ok(Modifier::MutRef)
        } else if input.peek(Token![ref]) {
            input.parse::<Token![ref]>()?;
            Ok(Modifier::Ref)
        } else if input.peek(Token![move]) {
            input.parse::<Token![move]>()?;
            Ok(Modifier::Move)
        } else {
            Err(input.error("expected one of: `ref`, `mut ref`, `move`"))
        }
    }
}

impl Into<TokenStream> for Modifier {
    fn into(self) -> TokenStream {
        match self {
            Modifier::Move => quote!(),
            Modifier::Ref => quote!(&),
            Modifier::MutRef => quote!(&mut),
        }
    }
}
