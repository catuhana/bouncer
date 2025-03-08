use darling::FromDeriveInput as _;
use quote::quote;

mod derive;

#[proc_macro_derive(Command, attributes(command, option))]
pub fn command_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = match syn::parse2::<syn::DeriveInput>(input.into()) {
        Ok(input) => input,
        Err(error) => return error.to_compile_error().into(),
    };

    let command = match derive::command::Command::from_derive_input(&input) {
        Ok(command) => command,
        Err(error) => return error.write_errors().into(),
    };

    quote! {
        #command
    }
    .into()
}
