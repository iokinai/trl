//! # helpers
//!
//! This module contains some helper functions

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Fields, Ident, Type, Visibility};

use crate::accessors::AccessorStructAttrs;

pub fn fill_includes_if_empty(includes: &mut Vec<String>, fields: &Fields) {
    if !includes.is_empty() {
        return;
    }

    for field in fields.iter() {
        includes.push(field.ident.clone().unwrap().to_string());
    }
}

pub fn should_add_pub(attrs: &AccessorStructAttrs, field: &Field) -> bool {
    if !attrs.include_pub {
        if let Visibility::Public(..) = field.vis {
            return false;
        }
    };

    true
}

/// Checks whether the field should be included based on the `includes` and `excludes` arguments
pub fn should_include(attrs: &AccessorStructAttrs, field: &Field) -> bool {
    if !attrs
        .includes
        .contains(&field.ident.clone().unwrap().to_string())
        || attrs
            .excludes
            .contains(&field.ident.clone().unwrap().to_string())
    {
        return false;
    }

    true
}

/// Checks whether the field should be included based on the `StructAttrs` struct
pub fn should_field_be_added(attrs: &AccessorStructAttrs, field: &Field) -> bool {
    should_add_pub(&attrs, &field) && should_include(&attrs, &field)
}

/// Generate a single getter `TokenStream`
pub fn generate_getter(
    name: &Ident,
    modifier: &TokenStream,
    field_name: &Ident,
    ty: &Type,
) -> TokenStream {
    quote! {
        pub fn #name(#modifier self) -> #modifier #ty {
            #modifier self.#field_name
        }
    }
}

/// Generates a single setter `TokenStream`
pub fn generate_setter(name: &Ident, field_name: &Ident, ty: &Type) -> TokenStream {
    quote! {
        pub fn #name(&mut self, value: #ty) {
            self.#field_name = value;
        }
    }
}
