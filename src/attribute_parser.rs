//! # attribute_parser
//! This module contains functions that parses the attribute

use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Attribute, Fields, ItemStruct, Meta, Token};

use crate::{
    arg::Arg,
    attribute_processors::{process_get, process_getters, process_set, process_setters},
    field_attrs::FieldAttrs,
    new_from_args::NewFromArgs,
    struct_attrs::StructAttrs,
};

/// Parses `Attribute` args into a punctuated sequence of `Arg`
pub fn parse_punctuated_attribute_args(attribute: &Attribute) -> Punctuated<Arg, Token![,]> {
    attribute
        .parse_args_with(Punctuated::<Arg, Token![,]>::parse_terminated)
        .expect("error on parse: ")
}

/// Creates attributes type with no arguments
pub fn get_argless_attrs<T>() -> T
where
    T: NewFromArgs,
{
    T::new(Punctuated::<Arg, Token![,]>::new())
}

/// Creates attributes type with specified arguments
pub fn get_attrs_for_args<T>(punctuated: Punctuated<Arg, Token![,]>) -> T
where
    T: NewFromArgs,
{
    T::new(punctuated)
}

/// Returns attribute name based on meta-type
pub fn match_attribute_name_attrs<T>(attrs: &mut T, name: &mut String, attribute: &Attribute)
where
    T: NewFromArgs,
{
    match &attribute.meta {
        Meta::Path(path) => {
            *name = path.get_ident().unwrap().to_string();
        }
        Meta::List(meta_list) => {
            *name = meta_list.path.get_ident().unwrap().to_string();
            let pun = parse_punctuated_attribute_args(attribute);

            *attrs = get_attrs_for_args::<T>(pun);
        }
        _ => {}
    };
}

/// Creates struct level attributes implementation
pub fn process_struct_attributes(
    attributes: &Vec<Attribute>,
    input: &ItemStruct,
    stream: &mut TokenStream,
) {
    for attribute in attributes {
        let mut attrs = get_argless_attrs::<StructAttrs>();
        let mut name = String::new();

        match_attribute_name_attrs::<StructAttrs>(&mut attrs, &mut name, &attribute);

        match name.as_str() {
            "getters" => stream.extend(process_getters(attrs, input)),
            "setters" => stream.extend(process_setters(attrs, input)),
            _ => {}
        };
    }
}

/// Creates field-level attributes implementation
pub fn process_field_attributes(fields: &Fields, stream: &mut TokenStream) {
    fields.iter().for_each(|field| {
        let attributes = &field.attrs;

        for attribute in attributes {
            let mut attrs = get_argless_attrs::<FieldAttrs>();
            let mut name = String::new();

            match_attribute_name_attrs(&mut attrs, &mut name, &attribute);

            match name.as_str() {
                "get" => stream.extend(process_get(attrs, field)),
                "set" => stream.extend(process_set(attrs, field)),
                _ => {}
            };
        }
    });
}

/// Generates impls
pub fn generate_impl_for_struct(input: &ItemStruct) -> proc_macro::TokenStream {
    let mut elements = TokenStream::new();

    process_struct_attributes(&input.attrs, &input, &mut elements);

    process_field_attributes(&input.fields, &mut elements);

    let struct_name = &input.ident;

    quote! {
        impl #struct_name {
            #elements
        }
    }
    .into()
}
