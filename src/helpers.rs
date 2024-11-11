use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprArray, ExprPath, Field, Fields, Ident, Type, Visibility};

use crate::{modifier::Modifier, struct_attrs::StructAttrs};

pub fn fill_includes_if_empty(includes: &mut Vec<String>, fields: &Fields) {
    if !includes.is_empty() {
        return;
    }

    for field in fields.iter() {
        includes.push(field.ident.clone().unwrap().to_string());
    }
}

pub fn should_add_pub(attrs: &StructAttrs, field: &Field) -> bool {
    if !attrs.include_pub {
        if let Visibility::Public(..) = field.vis {
            return false;
        }
    };

    true
}

pub fn should_include(attrs: &StructAttrs, field: &Field) -> bool {
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

pub fn should_field_be_added(attrs: &StructAttrs, field: &Field) -> bool {
    should_add_pub(&attrs, &field) && should_include(&attrs, &field)
}

pub fn path_to_string(expr: &Expr) -> String {
    if let Expr::Path(ExprPath { path, .. }) = expr {
        return path.get_ident().unwrap().to_string();
    };

    String::new()
}

pub fn parse_expr_array(value: &Expr, arr: &mut Vec<String>) {
    if let Expr::Array(ExprArray { elems, .. }) = value {
        elems.iter().for_each(|elem| {
            arr.push(path_to_string(elem));
        });
    }
}

pub fn parse_expr_str(value: &Expr, str: &mut String) {
    *str = path_to_string(value);
}

pub fn modifier_to_token_stream(modifier: &Modifier) -> TokenStream {
    match modifier {
        Modifier::Borrow => quote!(),
        Modifier::Ref => quote!(&),
        Modifier::MutRef => quote!(&mut),
    }
}

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

pub fn gernerate_setter(name: &Ident, field_name: &Ident, ty: &Type) -> TokenStream {
    quote! {
        pub fn #name(&mut self, value: #ty) {
            self.#field_name = value;
        }
    }
}
