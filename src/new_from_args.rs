//! # new_from_args
//!
//! This module contains the `NewFromArgs` trait which represents an attribute info struct that can be created from arguments

use syn::{punctuated::Punctuated, Token};

use crate::arg::Arg;

/// Represents an attribute info struct that can be created from arguments
pub trait NewFromArgs {
    fn new(args: Punctuated<Arg, Token![,]>) -> Self;
}
