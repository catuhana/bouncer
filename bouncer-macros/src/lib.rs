use attributes::{command::CommandAttributeFields, option::CommandOptionAttributeFields};

use proc_macro::TokenStream;
use syn::parse_macro_input;

extern crate proc_macro;

mod attributes;

#[proc_macro_derive(BouncerCommand, attributes(command, option))]
pub fn bouncer_command_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let command_attrs = match CommandAttributeFields::parse_attrs(&input) {
        Ok(attrs) => attrs,
        Err(error) => return error.to_compile_error().into(),
    };
    let option_attrs = match CommandOptionAttributeFields::parse_attrs(&input) {
        Ok(attrs) => attrs,
        Err(error) => return error.to_compile_error().into(),
    };

    TokenStream::default()
}
