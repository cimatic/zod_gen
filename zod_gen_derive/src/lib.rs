extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr};

/// Helper function to extract the serde rename value from variant attributes
///
/// Returns the renamed value if #[serde(rename = "...")] is found,
/// otherwise returns the variant name as a string.
fn extract_serde_rename(variant: &syn::Variant) -> String {
    for attr in &variant.attrs {
        if attr.path().is_ident("serde") {
            // Convert the attribute to a string and parse it manually
            let attr_str = quote!(#attr).to_string();

            // Look for rename pattern in the attribute string
            if let Some(rename_start) = attr_str.find("rename") {
                let rename_part = &attr_str[rename_start..];
                if let Some(quote_start) = rename_part.find('"') {
                    if let Some(quote_end) = rename_part[quote_start + 1..].find('"') {
                        let rename_value =
                            &rename_part[quote_start + 1..quote_start + 1 + quote_end];
                        return rename_value.to_string();
                    }
                }
            }
        }
    }
    // Fallback to variant name if no serde rename found
    variant.ident.to_string()
}

/// Derive macro for ZodSchema
#[proc_macro_derive(ZodSchema)]
pub fn derive_zod_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => {
                let fields = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    let field_name = LitStr::new(&ident.to_string(), ident.span());
                    let ty = &f.ty;
                    quote! { (#field_name, <#ty as zod_gen::ZodSchema>::zod_schema().as_str()) }
                });
                quote! {
                    impl zod_gen::ZodSchema for #name {
                        fn zod_schema() -> String {
                            zod_gen::zod_object(&[#(#fields),*])
                        }
                    }
                }
            }
            _ => panic!("ZodSchema derive only supports structs with named fields"),
        },
        Data::Enum(data_enum) => {
            let variants = data_enum.variants.iter().map(|v| {
                let renamed_value = extract_serde_rename(v);
                let var_name = LitStr::new(&renamed_value, v.ident.span());
                quote! { #var_name }
            });
            quote! {
                impl zod_gen::ZodSchema for #name {
                    fn zod_schema() -> String {
                        zod_gen::zod_enum(&[#(#variants),*])
                    }
                }
            }
        }
        _ => panic!("ZodSchema derive only supports structs and enums"),
    };

    TokenStream::from(expanded)
}
