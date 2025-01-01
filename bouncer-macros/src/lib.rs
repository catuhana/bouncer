// TODO: This needs a refactor and support both
// unit structs and enums (for subcommands).

use attributes::{command::CommandAttributeFields, option::CommandOptionAttributeFields};
use itertools::multiunzip;
use quote::quote;

extern crate proc_macro;

mod attributes;

#[proc_macro_derive(Command, attributes(command, option))]
pub fn command_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    command_derive_impl(input.into()).into()
}

#[expect(
    clippy::wildcard_imports,
    reason = "`quote!` macro uses wildcard imports internally"
)]
fn command_derive_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let input = syn::parse2::<syn::DeriveInput>(input).expect("Failed to parse input");

    let command_attrs = match CommandAttributeFields::parse_attrs(input.attrs.iter()) {
        Ok(attrs) => attrs,
        Err(error) => return error.to_compile_error(),
    };

    let option_attrs = match CommandOptionAttributeFields::parse_attrs(&input.data) {
        Ok(attrs) => attrs,
        Err(error) => return error.to_compile_error(),
    };

    let (option_builders, option_parsed, option_attr_idents): (Vec<_>, Vec<_>, Vec<_>) =
        multiunzip(option_attrs.fields.iter().map(|field| {
            (
                CommandOptionAttributeFields::generate_option_builder(field),
                CommandOptionAttributeFields::generate_option_parser(field),
                field.as_ident(),
            )
        }));

    let struct_name = &input.ident;
    let command_name = &command_attrs.name;
    let command_description = &command_attrs.description;

    quote! {
        impl bouncer_framework::command::CommandData for #struct_name {
            const COMMAND_NAME: &'static str = #command_name;
            const COMMAND_DESCRIPTION: &'static str = #command_description;

            fn command() -> Result<twilight_model::application::command::Command, bouncer_framework::command::CommandDataError> {
                let mut builder = Self::command_builder();
                #(#option_builders)*
                Ok(builder.validate()?.build())
            }
        }

        impl bouncer_framework::command::CommandOptions for #struct_name {
            fn parse_options(
                options: &[twilight_model::application::interaction::application_command::CommandDataOption],
            ) -> Result<Self, bouncer_framework::command::CommandOptionsError>
            {
                #(#option_parsed)*

                Ok(Self {
                    #(#option_attr_idents,)*
                })
            }
        }
    }
}
