//! # new_from_args
//!
//! This module contains the `NewFromArgs` trait which represents an attribute info struct that can be created from arguments

use syn::{Token, punctuated::Punctuated};

/// Represents an attribute info struct that can be created from arguments
pub trait NewFromArgs<TArg> {
    fn new(args: Punctuated<TArg, Token![,]>) -> Self;
}
