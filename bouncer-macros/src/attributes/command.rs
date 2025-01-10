use syn::spanned::Spanned as _;
use twilight_validate::command;

#[derive(Default)]
pub struct CommandAttributeFields {
    pub name: String,
    pub description: String,
}

impl syn::parse::Parse for CommandAttributeFields {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fields =
            syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated(
                input,
            )?;

        let mut name = String::default();
        let mut description = String::default();

        for field in fields {
            if let (
                Some(ident),
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }),
            ) = (
                field.path.get_ident().map(ToString::to_string).as_deref(),
                &field.value,
            ) {
                match ident {
                    "name" => {
                        name = lit_str.value();
                        Self::validate_name(&name, lit_str.span())?;
                    }
                    "description" => {
                        description = lit_str.value();
                        Self::validate_description(&description, lit_str.span())?;
                    }
                    _ => Err(syn::Error::new(
                        ident.span(),
                        "unknown command attribute field",
                    ))?,
                }
            }
        }

        Ok(Self { name, description })
    }
}

impl CommandAttributeFields {
    pub fn parse_attrs<'a>(
        mut attributes: impl Iterator<Item = &'a syn::Attribute>,
    ) -> syn::Result<Self> {
        attributes
            .find(|attr| attr.path().is_ident("command"))
            .ok_or_else(|| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "missing `#[command]` attribute on a `Command` derived struct",
                )
            })
            .and_then(syn::Attribute::parse_args)
    }

    fn validate_name(name: &str, span: proc_macro2::Span) -> syn::Result<()> {
        command::chat_input_name(name).map_err(|error| syn::Error::new(span, error))
    }

    fn validate_description(description: &str, span: proc_macro2::Span) -> syn::Result<()> {
        command::description(description).map_err(|error| syn::Error::new(span, error))
    }
}
