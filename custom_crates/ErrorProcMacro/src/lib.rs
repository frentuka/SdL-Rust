/* Main library file for the error_derive crate, which defines the Error derive macro. */

/* Import necessary dependencies for procedural macro development. */
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/* Define the Error derive macro. This macro automatically implements fmt::Display and
 * std::error::Error for enums. It ensures that the derive can only be applied to enums and
 * generates appropriate error formatting for each variant.
 *
 * Attributes:
 * - input: The input TokenStream representing the Rust code (e.g., an enum definition).
 *
 * Returns:
 * - A TokenStream containing the generated implementations.
 */
#[proc_macro_derive(Error)]
pub fn error_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Ensure it's an enum
    match &input.data {
        syn::Data::Enum(_) => {}
        _ => panic!("Error derive macro can only be applied to enums"),
    }

    let variants = if let syn::Data::Enum(data) = &input.data {
        &data.variants
    } else {
        unreachable!("Already checked for enum");
    };

    // Extract lifetime and generic parameters
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let display_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_str = variant_name.to_string();
        match &variant.fields {
            syn::Fields::Unit => quote! {
                #name::#variant_name => write!(f, #variant_str),
            },
            syn::Fields::Unnamed(fields) => {
                let field_count = fields.unnamed.len();
                let field_names = (0..field_count).map(|i| syn::Ident::new(&format!("field_{}", i), variant_name.span())).collect::<Vec<_>>();
                let placeholders = vec!["{}"; field_count].join(", ");
                let format_str = format!("{}({})", variant_str, placeholders);
                quote! {
                    #name::#variant_name(#(#field_names),*) => write!(f, #format_str, #(#field_names),*),
                }
            },
            syn::Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap()).collect::<Vec<_>>();
                let field_placeholders = field_names.iter().map(|n| format!("{} = {{}}", n)).collect::<Vec<_>>().join(", ");
                let format_str = format!("{}( {} )", variant_str, field_placeholders);
                quote! {
                    #name::#variant_name { #(#field_names),* } => write!(f, #format_str, #(#field_names),*),
                }
            },
        }
    });

    let expanded = quote! {
        impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(self, f)
            }
        }

        impl #impl_generics std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms)*
                }
            }
        }

        impl #impl_generics std::error::Error for #name #ty_generics #where_clause {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                None
            }
        }
    };

    TokenStream::from(expanded)
}