use darling::{FromDeriveInput, FromField, FromMeta as _};
use quote::{format_ident, quote};
use syn::spanned::Spanned as _;
use twilight_validate::command;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(command, option), supports(struct_named, struct_unit))]
pub struct Command {
    ident: syn::Ident,
    data: darling::ast::Data<darling::util::Ignored, CommandOptionField>,

    #[darling(with = Command::parse_command_name)]
    name: Option<String>,
    #[darling(with = Command::parse_command_description)]
    description: String,
}

#[derive(Debug)]
pub enum CommandOptionType {
    Boolean,
    String,
}

#[derive(Debug)]
pub enum CommandOption {
    Optional(CommandOptionType),
    Required(CommandOptionType),
}

#[derive(Debug, FromField)]
#[darling(attributes(option))]
pub struct CommandOptionField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(with = CommandOptionField::parse_command_option_name)]
    name: Option<String>,
    #[darling(with = CommandOptionField::parse_command_description)]
    description: String,
}

impl Command {
    fn parse_command_name(meta: &syn::Meta) -> darling::Result<Option<String>> {
        let command_name = String::from_meta(meta)?.to_lowercase();
        validate_command_chat_input_name(&command_name).map(|()| Some(command_name))
    }

    fn parse_command_description(meta: &syn::Meta) -> darling::Result<String> {
        let command_description = String::from_meta(meta)?;
        validate_command_description(&command_description).map(|()| command_description)
    }
}

impl CommandOptionField {
    fn parse_command_option_name(meta: &syn::Meta) -> darling::Result<Option<String>> {
        let option_name = String::from_meta(meta)?.to_lowercase();
        validate_option_name(&option_name).map(|()| Some(option_name))
    }

    fn parse_command_description(meta: &syn::Meta) -> darling::Result<String> {
        let option_description = String::from_meta(meta)?;
        validate_command_description(&option_description).map(|()| option_description)
    }

    fn generate_option_builders(&self) -> proc_macro2::TokenStream {
        let ident = self.ident.as_ref().unwrap();
        let field_name = ident.to_string().to_lowercase();
        let option_name = self.name.as_deref().unwrap_or(&field_name);
        let option_description = &self.description;

        let result = CommandOption::try_from(&self.ty);
        match result {
            Ok(option) => {
                let (option_type, required) = match option {
                    CommandOption::Optional(typ) => (typ, false),
                    CommandOption::Required(typ) => (typ, true),
                };

                let builder_type = format_ident!("{}Builder", option_type);
                quote! {
                    .option(
                        twilight_util::builder::command::#builder_type::new(#option_name, #option_description)
                            .required(#required)
                            .build()
                    )
                }
            }
            Err(error) => error.write_errors(),
        }
    }

    fn generate_option_parsers(&self) -> proc_macro2::TokenStream {
        let ident = self.ident.as_ref().unwrap();
        let field_name = ident.to_string().to_lowercase();
        let option_name = self.name.as_deref().unwrap_or(&field_name);
        let option_name_ident = format_ident!("{}", option_name);

        let result = CommandOption::try_from(&self.ty);
        let (option_type, required) = match result {
            Ok(option) => match option {
                CommandOption::Optional(typ) => (typ, false),
                CommandOption::Required(typ) => (typ, true),
            },
            Err(error) => return error.write_errors(),
        };

        if required {
            return quote! {
                let #option_name_ident = bouncer_framework::command::parse_required_option(
                    options,
                    #option_name,
                    |value| match value {
                        #option_type(value) => Some(value.to_owned()),
                        _ => None,
                    }
                )?;
            };
        }

        quote! {
            let #option_name_ident = bouncer_framework::command::parse_optional_option(
                options,
                #option_name,
                |value| match value {
                    #option_type(value) => Some(value.to_owned()),
                    _ => None,
                }
            )?;
        }
    }
}

impl quote::ToTokens for Command {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let ident_lowercase = ident.to_string().to_lowercase();

        let command_name = self.name.as_deref().unwrap_or(&ident_lowercase);
        let command_description = &self.description;

        let darling::ast::Data::Struct(fields) = &self.data else {
            unreachable!()
        };

        if fields.is_unit() {
            tokens.extend(quote! {
                impl bouncer_framework::command::CommandData for #ident {
                    const COMMAND_NAME: &'static str = #command_name;
                    const COMMAND_DESCRIPTION: &'static str = #command_description;

                    fn command() -> twilight_model::application::command::Command {
                        Self::command_builder().build()
                    }
                }
            });

            return ();
        }

        let mut option_builders = Vec::new();
        let mut option_parsers = Vec::new();
        let mut field_idents = Vec::new();

        for field in fields.iter() {
            option_builders.push(field.generate_option_builders());
            option_parsers.push(field.generate_option_parsers());
            field_idents.push(field.ident.as_ref());
        }

        tokens.extend(quote! {
            impl bouncer_framework::command::CommandData for #ident {
                const COMMAND_NAME: &'static str = #command_name;
                const COMMAND_DESCRIPTION: &'static str = #command_description;

                fn command() -> twilight_model::application::command::Command {
                    Self::command_builder()
                        #(#option_builders)*
                        .build()
                }
            }

            impl bouncer_framework::command::CommandOptions for #ident {
                fn parse_options(
                    options: &[twilight_model::application::interaction::application_command::CommandDataOption],
                ) -> Result<Self, bouncer_framework::command::CommandOptionsError>
                {
                    #(#option_parsers)*

                    Ok(Self {
                        #(#field_idents),*
                    })
                }
            }
        });
    }
}

fn validate_command_chat_input_name(name: &str) -> darling::Result<()> {
    match command::chat_input_name(name) {
        Ok(()) => Ok(()),
        Err(error) => Err(darling::Error::custom(error)),
    }
}

fn validate_option_name(name: &str) -> darling::Result<()> {
    match command::option_name(name) {
        Ok(()) => Ok(()),
        Err(error) => Err(darling::Error::custom(error)),
    }
}

fn validate_command_description(description: &str) -> darling::Result<()> {
    match command::description(description) {
        Ok(()) => Ok(()),
        Err(error) => Err(darling::Error::custom(error)),
    }
}

fn extract_path_segment(ty: &syn::Type) -> Result<&syn::PathSegment, syn::Error> {
    let span = ty.span();

    let syn::Type::Path(type_path) = ty else {
        return Err(syn::Error::new(span, "Expected a path type"));
    };

    type_path
        .path
        .segments
        .first()
        .ok_or_else(|| syn::Error::new(span, "Missing type name"))
}

impl TryFrom<&syn::Type> for CommandOption {
    type Error = darling::Error;

    fn try_from(value: &syn::Type) -> Result<Self, Self::Error> {
        let path_segment = extract_path_segment(value)?;
        let ident = &path_segment.ident;

        if ident == "Option" {
            if let syn::PathArguments::AngleBracketed(args) = &path_segment.arguments {
                if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                    return Ok(Self::Optional(CommandOptionType::try_from(inner_type)?));
                }
            }

            return Err(
                darling::Error::custom("Invalid Option type arguments").with_span(&ident.span())
            );
        }

        Ok(Self::Required(CommandOptionType::try_from(value)?))
    }
}

impl TryFrom<&syn::Type> for CommandOptionType {
    type Error = darling::Error;

    fn try_from(value: &syn::Type) -> Result<Self, Self::Error> {
        let path_segment = extract_path_segment(value)?;
        let type_name = path_segment.ident.to_string();

        match &type_name[..] {
            "bool" => Ok(Self::Boolean),
            "String" => Ok(Self::String),
            _ => Err(
                darling::Error::custom(format!("Unsupported option type `{type_name}`"))
                    .with_span(&path_segment.ident.span()),
            ),
        }
    }
}

impl core::fmt::Display for CommandOptionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Boolean => write!(f, "Boolean"),
            Self::String => write!(f, "String"),
        }
    }
}

impl quote::ToTokens for CommandOptionType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Boolean => {
                quote!(twilight_model::application::interaction::application_command::CommandOptionValue::Boolean)
            }
            Self::String => {
                quote!(twilight_model::application::interaction::application_command::CommandOptionValue::String)
            }
        }
        .to_tokens(tokens);
    }
}

impl quote::IdentFragment for CommandOptionType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{self}")
    }
}
