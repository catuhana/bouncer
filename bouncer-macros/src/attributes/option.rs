use quote::{format_ident, quote};
use syn::spanned::Spanned as _;
use twilight_validate::command;

pub struct CommandOptionAttributeFields {
    pub fields: Vec<CommandOptionField>,
}

#[derive(Default)]
pub struct CommandOptionField {
    pub name: String,
    pub description: String,
    pub r#type: CommandOptionType,
    pub required: bool,
}

#[derive(Default)]
pub enum CommandOptionType {
    #[default]
    Boolean,
    Channel,
    Integer,
    User,
    Role,
    String,
}

impl CommandOptionField {
    fn parse_description(field: &syn::MetaNameValue) -> Option<String> {
        if !field.path.is_ident("description") {
            return None;
        }

        match &field.value {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) => Some(lit_str.value()),
            _ => None,
        }
    }

    fn validate_description(description: &str, span: proc_macro2::Span) -> syn::Result<()> {
        command::description(description).map_err(|error| syn::Error::new(span, error))
    }
}

impl syn::parse::Parse for CommandOptionField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields =
            syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated(
                input,
            )?;

        let mut description = String::default();

        for field in fields {
            if let Some(desc) = Self::parse_description(&field) {
                description = desc;
                Self::validate_description(&description, field.value.span())?;
            }
        }

        Ok(Self {
            description,
            ..Default::default()
        })
    }
}

impl CommandOptionAttributeFields {
    pub fn parse_attrs(input: &syn::DeriveInput) -> syn::Result<Self> {
        let syn::Data::Struct(data) = &input.data else {
            return Ok(Self { fields: Vec::new() });
        };

        let syn::Fields::Named(named_fields) = &data.fields else {
            return Ok(Self { fields: Vec::new() });
        };

        let mut fields = Vec::new();
        for field in &named_fields.named {
            if let Some(attr) = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("option"))
            {
                let mut option = attr.parse_args::<CommandOptionField>()?;
                let (type_, required) = Self::parse_type(&field.ty)?;

                option.name = field
                    .ident
                    .as_ref()
                    .map(std::string::ToString::to_string)
                    .ok_or_else(|| {
                        syn::Error::new(field.span(), "Unnamed fields are not supported.")
                    })?;
                Self::validate_option_name(&option.name, field.span())?;

                option.r#type = type_;
                option.required = required;

                fields.push(option);
            }
        }

        let result_struct = Self { fields };
        result_struct.validate_option_order()?;

        Ok(result_struct)
    }

    fn parse_type(r#type: &syn::Type) -> syn::Result<(CommandOptionType, bool)> {
        let syn::Type::Path(type_path) = r#type else {
            return Err(syn::Error::new(r#type.span(), "Unsupported type."));
        };

        let segment = type_path
            .path
            .segments
            .last()
            .ok_or_else(|| syn::Error::new(r#type.span(), "Invalid type."))?;

        if segment.ident == "Option" {
            return Self::parse_option_type(segment, r#type);
        }

        Ok((Self::get_inner_type(r#type)?, true))
    }

    fn parse_option_type(
        segment: &syn::PathSegment,
        r#type: &syn::Type,
    ) -> syn::Result<(CommandOptionType, bool)> {
        let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
            return Err(syn::Error::new(r#type.span(), "Invalid Option type."));
        };

        let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() else {
            return Err(syn::Error::new(r#type.span(), "Invalid Option type."));
        };

        Ok((Self::get_inner_type(inner_type)?, false))
    }

    fn get_inner_type(r#type: &syn::Type) -> syn::Result<CommandOptionType> {
        let syn::Type::Path(type_path) = r#type else {
            return Err(syn::Error::new(r#type.span(), "Unsupported type."));
        };

        let segment = type_path
            .path
            .segments
            .last()
            .ok_or_else(|| syn::Error::new(r#type.span(), "Invalid type."))?;

        match segment.ident.to_string().as_str() {
            "bool" => Ok(CommandOptionType::Boolean),
            "i64" => Ok(CommandOptionType::Integer),
            "String" => Ok(CommandOptionType::String),
            "Id" => Self::parse_id_type(segment, r#type),
            _ => Err(syn::Error::new(r#type.span(), "Unsupported type.")),
        }
    }

    fn parse_id_type(
        segment: &syn::PathSegment,
        r#type: &syn::Type,
    ) -> syn::Result<CommandOptionType> {
        let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
            return Err(syn::Error::new(r#type.span(), "Invalid Id type."));
        };

        let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() else {
            return Err(syn::Error::new(r#type.span(), "Invalid Id type."));
        };

        let syn::Type::Path(inner_path) = inner_type else {
            return Err(syn::Error::new(inner_type.span(), "Invalid type."));
        };

        let inner_segment = inner_path
            .path
            .segments
            .last()
            .ok_or_else(|| syn::Error::new(inner_type.span(), "Invalid type."))?;

        match inner_segment.ident.to_string().as_str() {
            "ChannelMarker" => Ok(CommandOptionType::Channel),
            "UserMarker" => Ok(CommandOptionType::User),
            "RoleMarker" => Ok(CommandOptionType::Role),
            _ => Err(syn::Error::new(inner_type.span(), "Unsupported type.")),
        }
    }

    fn validate_option_name(name: &str, span: proc_macro2::Span) -> syn::Result<()> {
        command::option_name(name).map_err(|error| syn::Error::new(span, error))
    }

    fn validate_option_order(&self) -> syn::Result<()> {
        let mut found_optional = false;
        let mut invalid_required_fields = Vec::new();

        for field in &self.fields {
            if !field.required {
                found_optional = true;
                continue;
            }

            if found_optional {
                invalid_required_fields.push(field.name.clone());
            }
        }

        if !invalid_required_fields.is_empty() {
            let mut error = syn::Error::new(
                invalid_required_fields[0].span(),
                format!(
                    "Required option '{}' must be placed before optional fields",
                    invalid_required_fields[0]
                ),
            );

            for field in invalid_required_fields.iter().skip(1) {
                error.combine(syn::Error::new(
                    field.span(),
                    format!(
                        "Required option '{}' must be placed before optional fields",
                        field
                    ),
                ));
            }

            return Err(error);
        }

        Ok(())
    }

    pub fn generate_option_builder(field: &CommandOptionField) -> proc_macro2::TokenStream {
        let name = &field.name;
        let description = &field.description;
        let set_required = field.required.then(|| quote! { .required(true) });

        let builder_name = format_ident!(
            "{}Builder",
            match field.r#type {
                CommandOptionType::String => "String",
                CommandOptionType::Boolean => "Boolean",
                CommandOptionType::Channel => "Channel",
                CommandOptionType::Integer => "Integer",
                CommandOptionType::Role => "Role",
                CommandOptionType::User => "User",
            }
        );

        quote! {
            builder = builder.option(
                twilight_util::builder::command::#builder_name::new(#name, #description)
                #set_required
                .build()
            );
        }
    }

    pub fn generate_option_parser(field: &CommandOptionField) -> proc_macro2::TokenStream {
        let name = &field.name;
        let name_ident = format_ident!("{}", name);
        let unwrap_non_option = field
            .required
            .then(|| quote! { .ok_or(bouncer_framework::command::CommandOptionsError::MissingRequiredOption(String::from(#name)))? });

        let generate_parser = |value_type: proc_macro2::TokenStream,
                               value_pat: proc_macro2::TokenStream| {
            quote! {
                let #name_ident = options
                    .iter()
                    .find(|option| option.name == #name)
                    .map(|option| match &option.value {
                        twilight_model::application::interaction::application_command::CommandOptionValue::#value_type(#value_pat) => Ok(#value_pat.to_owned()),
                        option => return Err(bouncer_framework::command::CommandOptionsError::UnexpectedOptionType(String::from(#name), option.to_owned())),
                    })#unwrap_non_option?;
            }
        };

        match field.r#type {
            CommandOptionType::String => generate_parser(quote!(String), quote!(string)),
            CommandOptionType::Integer => generate_parser(quote!(Integer), quote!(integer)),
            CommandOptionType::Boolean => generate_parser(quote!(Boolean), quote!(boolean)),
            CommandOptionType::User => generate_parser(quote!(User), quote!(user)),
            CommandOptionType::Channel => generate_parser(quote!(Channel), quote!(channel)),
            CommandOptionType::Role => generate_parser(quote!(Role), quote!(role)),
        }
    }
}
