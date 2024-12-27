use attributes::command::CommandAttributeFields;

use proc_macro::TokenStream;
use syn::parse_macro_input;

extern crate proc_macro;

mod attributes;

#[proc_macro_derive(BouncerCommand, attributes(command))]
pub fn bouncer_command_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let command_attrs = match CommandAttributeFields::parse_attrs(&input) {
        Ok(attrs) => attrs,
        Err(error) => return error.to_compile_error().into(),
    };

    // let struct_name = &input.ident;

    // TokenStream::from(expanded)

    TokenStream::default()
}
