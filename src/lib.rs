use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Error};

#[proc_macro_derive(Macro)]
pub fn derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let span = input.generics.span();
    Error::new(span, "oops")
        .to_compile_error()
        .into()
}
