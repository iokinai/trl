use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, ItemStruct, Meta, Token};
use trl_global::{fill_includes_if_empty, should_field_be_added, Attrs, Modifier};

#[proc_macro_attribute]
pub fn getters(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let punctuated = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

    let mut attrs = Attrs::new(punctuated);

    if attrs.includes.is_empty() {
        fill_includes_if_empty(&mut attrs.includes, &input.fields);
    }

    let struct_name = &input.ident;
    let getters = &input
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            if !should_field_be_added(&attrs, &field) {
                return TokenStream::new();
            }

            let field_name_str = &field.ident.clone().unwrap().to_string();
            let getter_name = format_ident!("{}{}", attrs.prefix, field_name_str);

            let modifier = match attrs.modifier {
                Modifier::Borrow => quote!(),
                Modifier::Ref => quote!(&),
                Modifier::MutRef => quote!(&mut),
            };

            quote! {
                pub fn #getter_name(#modifier self) -> #modifier #field_type {
                    #modifier self.#field_name
                }
            }
        })
        .collect::<Vec<TokenStream>>();

    let expanded = quote! {
        #input
        impl #struct_name {
            #(#getters)*
        }
    };

    proc_macro::TokenStream::from(expanded)
}
