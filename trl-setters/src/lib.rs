use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, ItemStruct, Meta, Token};
use trl_global::{fill_includes_if_empty, should_field_be_added, Attrs};

#[proc_macro_attribute]
pub fn setters(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let punctuated = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut attrs = Attrs::new(punctuated);

    if attrs.includes.is_empty() {
        fill_includes_if_empty(&mut attrs.includes, &input.fields);
    }

    if attrs.prefix.is_empty() {
        attrs.prefix = String::from("set_");
    }

    let struct_name = &input.ident;
    let setters = &input
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            if !should_field_be_added(&attrs, &field) {
                return TokenStream::new();
            }

            let field_name_str = &field.ident.clone().unwrap().to_string();
            let setter_name = format_ident!("{}{}", attrs.prefix, field_name_str);

            quote! {
                pub fn #setter_name(&mut self, value: #field_type) {
                    self.#field_name = value;
                }
            }
        })
        .collect::<Vec<TokenStream>>();

    dbg!(setters[1].to_string());

    let expanded = quote! {
        #input
        impl #struct_name {
            #(#setters)*
        }
    };

    proc_macro::TokenStream::from(expanded)
}
