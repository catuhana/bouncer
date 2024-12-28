use attributes::{command::CommandAttributeFields, option::CommandOptionAttributeFields};
use itertools::multiunzip;
use quote::quote;

extern crate proc_macro;

mod attributes;

#[proc_macro_derive(BouncerCommand, attributes(command, option))]
pub fn bouncer_command_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bouncer_command_derive_impl(input.into()).into()
}

fn bouncer_command_derive_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
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
            {
                #(#option_parsed)*

                Ok(Self {
                    #(#option_attr_idents,)*
                })
            }
        }
    }
}
