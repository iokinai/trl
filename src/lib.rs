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
