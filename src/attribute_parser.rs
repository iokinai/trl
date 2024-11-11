use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Attribute, Fields, ItemStruct, Meta, Token};

use crate::{
    attribute_processors::{process_get, process_getters, process_set, process_setters},
    field_attrs::FieldAttrs,
    new_from_args::NewFromArgs,
    struct_attrs::StructAttrs,
};

pub(crate) fn parse_punctuated_attribute_args(
    attribute: &Attribute,
) -> Punctuated<Meta, Token![,]> {
    attribute
        .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
        .unwrap()
}

pub(crate) fn get_argless_attrs<T>() -> T
where
    T: NewFromArgs,
{
    T::new(Punctuated::<Meta, Token![,]>::new())
}

pub(crate) fn get_attrs_for_args<T>(punctuated: Punctuated<Meta, Token![,]>) -> T
where
    T: NewFromArgs,
{
    T::new(punctuated)
}

pub(crate) fn match_attribute_name_attrs<T>(attrs: &mut T, name: &mut String, attribute: &Attribute)
where
    T: NewFromArgs,
{
    match &attribute.meta {
        Meta::Path(path) => {
            *name = path.get_ident().unwrap().to_string();
        }
        Meta::List(meta_list) => {
            *name = meta_list.path.get_ident().unwrap().to_string();
            *attrs = get_attrs_for_args::<T>(parse_punctuated_attribute_args(attribute));
        }
        _ => {}
    };
}

pub(crate) fn process_struct_attributes(
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

pub(crate) fn process_field_attributes(fields: &Fields, stream: &mut TokenStream) {
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
