use attributes::{command::CommandAttributeFields, option::CommandOptionAttributeFields};

use itertools::multiunzip;
use quote::{format_ident, quote};
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
    let (option_builders, option_parsed, option_attr_idents): (
        Vec<_>,
        Vec<_>,
        Vec<proc_macro2::Ident>,
    ) = multiunzip(option_attrs.fields.iter().map(|attr| {
        (
            CommandOptionAttributeFields::generate_option_builder(attr),
            CommandOptionAttributeFields::generate_option_parser(attr),
            format_ident!("{}", &attr.name),
        )
    }));

    let struct_name = &input.ident;

    let command_name = &command_attrs.name;
    let command_description = &command_attrs.description;

    let expanded = quote! {
        impl bouncer_framework::command::CommandData for #struct_name {
            const COMMAND_NAME: &'static str = #command_name;
            const COMMAND_DESCRIPTION: &'static str = #command_description;

            fn command() -> twilight_model::application::command::Command {
                let mut builder = Self::command_builder();
                #(#option_builders)*
                builder.build()
            }
        }

        impl bouncer_framework::command::CommandOptions for #struct_name {
            fn parse_options(
                options: &[twilight_model::application::interaction::application_command::CommandDataOption],
            ) -> Result<Self, bouncer_framework::command::CommandOptionsError>
            where
                Self: Sized,
            {
                #(#option_parsed)*

                Ok(Self {
                    #(#option_attr_idents,)*
                })
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
