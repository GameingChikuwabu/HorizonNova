use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

use proc_macro::TokenStream;

pub fn derive_error_impl(input: TokenStream)-> TokenStream{
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Generate Display implementation based on enum variants
    let display_impl = match input.data {
        Data::Enum(data_enum) => {
            let arms = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                match &variant.fields {
                    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        // For enums like `ChankError(String)`
                        quote! {
                            Self::#variant_name(ref msg) => write!(f, "{}: {}", stringify!(#variant_name), msg),
                        }
                    }
                    Fields::Unnamed(_) | Fields::Named(_) => {
                        // Handle other unnamed or named fields if needed
                        quote! {
                            Self::#variant_name(..) => write!(f, "{}", stringify!(#variant_name)),
                        }
                    }
                    Fields::Unit => {
                        // For unit-like variants like `InvalidInput`
                        quote! {
                            Self::#variant_name => write!(f, "{}", stringify!(#variant_name)),
                        }
                    }
                }
            });

            quote! {
                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(#arms)*
                        }
                    }
                }
            }
        }
        _ => quote! {
            compile_error!("MyError can only be derived for enums");
        },
    };

    // Combine the Display implementation with the Error trait implementation
    let expanded = quote! {
        #display_impl

        impl std::error::Error for #name {}
    };

    TokenStream::from(expanded)
}