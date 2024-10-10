mod impls{
    pub mod error_macros;
}
use proc_macro::TokenStream;


#[proc_macro_derive(Error,attributes(error))]
pub fn derive_error(input: TokenStream) -> TokenStream {
    crate::impls::error_macros::derive_error_impl(input)
}