//! # attribute_parser
//! This module contains functions that parses the attribute

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Fields, ItemStruct, Meta, Token, parse::Parse, punctuated::Punctuated};

use crate::{
    accessors::{
        AccessorArg, AccessorFieldAttrs, AccessorStructAttrs,
        accessor_processor::{process_get, process_getters, process_set, process_setters},
    },
    constructor::{
        ConstructorArg, ConstructorStructAttrs, constructor_processor::process_constructor,
    },
    new_from_args::NewFromArgs,
};

/// Parses `Attribute` args into a punctuated sequence of `Arg`
pub fn parse_punctuated_attribute_args<TArg>(attribute: &Attribute) -> Punctuated<TArg, Token![,]>
where
    TArg: Parse,
{
    attribute
        .parse_args_with(Punctuated::<TArg, Token![,]>::parse_terminated)
        .expect("error on parse: ")
}

/// Creates attributes type with specified arguments
pub fn get_attrs_for_args<T, TArgs>(punctuated: Punctuated<TArgs, Token![,]>) -> T
where
    T: NewFromArgs<TArgs>,
{
    T::new(punctuated)
}

/// Returns attribute name based on meta-type
// pub fn match_attribute_name_attrs<T, TArgs>(attrs: &mut T, name: &mut String, attribute: &Attribute)
// where
//     T: NewFromArgs<TArgs>,
//     TArgs: Parse,
// {
//     match &attribute.meta {
//         Meta::Path(path) => {
//             *name = path.get_ident().unwrap().to_string();
//         }
//         Meta::List(meta_list) => {
//             *name = meta_list.path.get_ident().unwrap().to_string();
//             let pun = parse_punctuated_attribute_args(attribute);

//             *attrs = get_attrs_for_args::<T, TArgs>(pun);
//         }
//         _ => {}
//     };
// }

/// This function returns the `Attribute` name
pub fn get_attribute_name(attribute: &Attribute) -> String {
    let _expected_attribute_name = "Expected attribute name.\nNote: attributes must be *directly imported* and it's names can not be changed. You should only use #[constructor], not #[trl::constructor]";

    match &attribute.meta {
        Meta::Path(path) => path
            .get_ident()
            .expect(_expected_attribute_name)
            .to_string(),
        Meta::List(meta_list) => meta_list
            .path
            .get_ident()
            .expect(_expected_attribute_name)
            .to_string(),
        _ => unreachable!(),
    }
}

/// This function loads args from `Attribute`
pub fn load_args_from_attribute<T, TArgs>(attribute: &Attribute) -> T
where
    T: NewFromArgs<TArgs>,
    TArgs: Parse,
{
    if let Meta::List(_) = &attribute.meta {
        let pun = parse_punctuated_attribute_args(attribute);
        get_attrs_for_args::<T, TArgs>(pun)
    } else {
        T::new(Punctuated::<TArgs, Token![,]>::new())
    }
}

/// Creates struct level attributes implementation
pub fn process_struct_attributes(
    attributes: &Vec<Attribute>,
    input: &ItemStruct,
    stream: &mut TokenStream,
) {
    for attribute in attributes {
        let name = get_attribute_name(attribute);

        match name.as_str() {
            "getters" => {
                let attrs = load_args_from_attribute::<AccessorStructAttrs, AccessorArg>(attribute);
                // match_attribute_name_attrs::<AccessorStructAttrs, AccessorArg>(
                //     &mut attrs, &mut name, &attribute,
                // );
                stream.extend(process_getters(attrs, input))
            }
            "setters" => {
                let attrs = load_args_from_attribute::<AccessorStructAttrs, AccessorArg>(attribute);
                // match_attribute_name_attrs::<AccessorStructAttrs, AccessorArg>(
                //     &mut attrs, &mut name, &attribute,
                // );
                stream.extend(process_setters(attrs, input))
            }
            "constructor" => {
                let attrs =
                    load_args_from_attribute::<ConstructorStructAttrs, ConstructorArg>(attribute);

                stream.extend(process_constructor(attrs, input));
            }
            _ => {}
        };
    }
}

/// Creates field-level attributes implementation
pub fn process_field_attributes(fields: &Fields, stream: &mut TokenStream) {
    fields.iter().for_each(|field| {
        let attributes = &field.attrs;

        for attribute in attributes {
            let name = get_attribute_name(attribute);

            match name.as_str() {
                "get" => {
                    let attrs =
                        load_args_from_attribute::<AccessorFieldAttrs, AccessorArg>(attribute);
                    // match_attribute_name_attrs::<AccessorFieldAttrs, AccessorArg>(
                    //     &mut attrs, &mut name, &attribute,
                    // );
                    stream.extend(process_get(attrs, field))
                }
                "set" => {
                    let attrs =
                        load_args_from_attribute::<AccessorFieldAttrs, AccessorArg>(attribute);
                    // match_attribute_name_attrs::<AccessorFieldAttrs, AccessorArg>(
                    //     &mut attrs, &mut name, &attribute,
                    // );
                    stream.extend(process_set(attrs, field))
                }
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
