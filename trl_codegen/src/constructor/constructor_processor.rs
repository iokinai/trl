use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ItemStruct;

use crate::constructor::{
    constructor_struct_attrs::ConstructorStructAttrs, constructor_visibility::ConstructorVisibility,
};

pub fn process_constructor(attrs: ConstructorStructAttrs, input: &ItemStruct) -> TokenStream {
    let constructor_visibility = match attrs.visibility {
        ConstructorVisibility::Pub => quote! {pub},
        ConstructorVisibility::PubPath(path) => {
            let inn_path = format_ident!("{}", path);
            quote! {pub(#inn_path)}
        }
        ConstructorVisibility::Private => quote! {},
    };

    let constructor_name = format_ident!("{}", attrs.name);

    let mut constructor_args = TokenStream::new();
    let mut constructor_values = TokenStream::new();
    input.fields.iter().for_each(|field| {
        let field_name = field.clone().ident.expect("Error, expected named field");
        let field_ty = field.clone().ty;

        constructor_args.extend(quote! {
            #field_name: #field_ty,
        });

        constructor_values.extend(quote! {
            #field_name,
        });
    });

    quote! {
        #constructor_visibility fn #constructor_name(#constructor_args) -> Self {
            Self {#constructor_values}
        }
    }
}
