//! # accessor_processor
//! This module contains functions that generate method from the provided information

use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{Field, ItemStruct};

use crate::{
    accessors::AccessorFieldAttrs,
    accessors::AccessorStructAttrs,
    helpers::{fill_includes_if_empty, generate_getter, generate_setter, should_field_be_added},
};

/// Generates `getters` `TokenStream` based on the provided `AccessorStructAttrs`
pub fn process_getters(mut attrs: AccessorStructAttrs, input: &ItemStruct) -> TokenStream {
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

/// Generates `setters` `TokenStream` based on the provided `AccessorStructAttrs`
pub fn process_setters(mut attrs: AccessorStructAttrs, input: &ItemStruct) -> TokenStream {
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

/// Generates `get` `TokenStream` based on the provided `AccessorFieldAttrs`
pub fn process_get(attrs: AccessorFieldAttrs, field: &Field) -> TokenStream {
    let fin = &field.ident.clone().unwrap();
    let ty = &field.ty;

    let mut getter_name = fin.clone();

    if !attrs.name.is_empty() {
        getter_name = format_ident!("{}", attrs.name);
    }

    if !attrs.prefix.is_empty() {
        getter_name = format_ident!("{}{}", attrs.prefix, getter_name);
    }

    let modifier = &attrs.modifier.into();

    generate_getter(&getter_name, &modifier, &fin, ty)
}

/// Generates `set` `TokenStream` based on the provided `AccessorFieldAttrs`
pub fn process_set(mut attrs: AccessorFieldAttrs, field: &Field) -> TokenStream {
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

    generate_setter(&setter_name, fin, ty)
}
