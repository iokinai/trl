use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{Field, ItemStruct};

use crate::{
    field_attrs::FieldAttrs,
    helpers::{
        fill_includes_if_empty, generate_getter, gernerate_setter, modifier_to_token_stream,
        should_field_be_added,
    },
    struct_attrs::StructAttrs,
};

pub(crate) fn process_getters(mut attrs: StructAttrs, input: &ItemStruct) -> TokenStream {
    fill_includes_if_empty(&mut attrs.includes, &input.fields);

    let mut result = TokenStream::new();

    input.fields.iter().for_each(|field| {
        if !should_field_be_added(&attrs, &field) {
            return;
        }

        result.extend(process_get(attrs.clone().into(), field));
    });

    result
}

pub(crate) fn process_setters(mut attrs: StructAttrs, input: &ItemStruct) -> TokenStream {
    fill_includes_if_empty(&mut attrs.includes, &input.fields);

    if attrs.prefix.is_empty() {
        attrs.prefix = String::from("set_");
    }

    let mut result = TokenStream::new();

    input.fields.iter().for_each(|field| {
        if !should_field_be_added(&attrs, &field) {
            return;
        }

        result.extend(process_set(attrs.clone().into(), field));
    });

    result
}

pub(crate) fn process_get(attrs: FieldAttrs, field: &Field) -> TokenStream {
    let fin = &field.ident.clone().unwrap();
    let ty = &field.ty;

    let mut getter_name = fin.clone();

    if !attrs.name.is_empty() {
        getter_name = format_ident!("{}", attrs.name);
    }

    if !attrs.prefix.is_empty() {
        getter_name = format_ident!("{}{}", attrs.prefix, getter_name);
    }

    let modifier = modifier_to_token_stream(&attrs.modifier);

    generate_getter(&getter_name, &modifier, &fin, ty)
}

pub(crate) fn process_set(mut attrs: FieldAttrs, field: &Field) -> TokenStream {
    let fin = &field.ident.clone().unwrap();
    let ty = &field.ty;

    if attrs.prefix.is_empty() {
        attrs.prefix = String::from("set_");
    }

    let mut setter_name = fin.clone();

    if !attrs.name.is_empty() {
        setter_name = format_ident!("{}", attrs.name);
    }

    setter_name = format_ident!("{}{}", attrs.prefix, setter_name);

    gernerate_setter(&setter_name, fin, ty)
}
