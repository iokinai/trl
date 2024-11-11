use syn::{punctuated::Punctuated, Meta, Token};

pub trait NewFromArgs {
    fn new(args: Punctuated<Meta, Token![,]>) -> Self;
}
