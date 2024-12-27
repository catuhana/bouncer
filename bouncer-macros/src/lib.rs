use attributes::{command::CommandAttributeFields, option::CommandOptionAttributeFields};

use quote::quote;
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
    let _option_attrs = match CommandOptionAttributeFields::parse_attrs(&input) {
        Ok(attrs) => attrs,
        Err(error) => return error.to_compile_error().into(),
    };

    let struct_name = &input.ident;

    let command_name = &command_attrs.name;
    let command_description = &command_attrs.description;

    let expanded = quote! {
        #[async_trait::async_trait]
        impl bouncer_framework::command::Command for #struct_name {
            const COMMAND_NAME: &'static str = #command_name;
            const COMMAND_DESCRIPTION: &'static str = #command_description;

            async fn execute(&self) -> Result<(), bouncer_framework::command::CommandExecuteError> {
                todo!()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
