use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, I};


#[proc_macro_attribute]
pub fn component(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let output = quote! {
        #input
        impl Component for #input {}
    };
    output.into()
}