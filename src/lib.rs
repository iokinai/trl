mod attribute_parser;
mod attribute_processors;
mod field_attrs;
mod helpers;
mod modifier;
mod new_from_args;
mod struct_attrs;

use attribute_parser::generate_impl_for_struct;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(trl, attributes(get, set))]
pub fn trl_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    generate_impl_for_struct(&input)
}

#[proc_macro_attribute]
pub fn getters(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

#[proc_macro_attribute]
pub fn setters(
    _: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}

// #[proc_macro_attribute]
// pub fn get(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     dbg!(item.to_string());
//     item
// }

// #[proc_macro_attribute]
// pub fn getters(
//     attr: proc_macro::TokenStream,
//     item: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
//     let input = parse_macro_input!(item as ItemStruct);

//     let punctuated = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated);

//     let mut attrs = Attrs::new(punctuated);

//     if attrs.includes.is_empty() {
//         fill_includes_if_empty(&mut attrs.includes, &input.fields);
//     }

//     let struct_name = &input.ident;
//     let getters = &input
//         .fields
//         .iter()
//         .map(|field| {
//             let field_name = &field.ident;
//             let field_type = &field.ty;

//             if !should_field_be_added(&attrs, &field) {
//                 return TokenStream::new();
//             }

//             let field_name_str = &field.ident.clone().unwrap().to_string();
//             let getter_name = format_ident!("{}{}", attrs.prefix, field_name_str);

//             let modifier = match attrs.modifier {
//                 Modifier::Borrow => quote!(),
//                 Modifier::Ref => quote!(&),
//                 Modifier::MutRef => quote!(&mut),
//             };

//             quote! {
//                 pub fn #getter_name(#modifier self) -> #modifier #field_type {
//                     #modifier self.#field_name
//                 }
//             }
//         })
//         .collect::<Vec<TokenStream>>();

//     let expanded = quote! {
//         #input
//         impl #struct_name {
//             #(#getters)*
//         }
//     };

//     proc_macro::TokenStream::from(expanded)
// }
