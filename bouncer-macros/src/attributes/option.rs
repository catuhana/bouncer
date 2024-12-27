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
    String,
    Integer,
    Number,
    Boolean,
    User,
    Channel,
    Role,
    Attachment,
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

        Ok(Self { fields })
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
            "String" => Ok(CommandOptionType::String),
            "i64" => Ok(CommandOptionType::Integer),
            "f64" => Ok(CommandOptionType::Number),
            "bool" => Ok(CommandOptionType::Boolean),
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
            "UserMarker" => Ok(CommandOptionType::User),
            "ChannelMarker" => Ok(CommandOptionType::Channel),
            "RoleMarker" => Ok(CommandOptionType::Role),
            "AttachmentMarker" => Ok(CommandOptionType::Attachment),
            _ => Err(syn::Error::new(inner_type.span(), "Unsupported type.")),
        }
    }

    fn validate_option_name(name: &str, span: proc_macro2::Span) -> syn::Result<()> {
        command::option_name(name).map_err(|error| syn::Error::new(span, error))
    }
}
