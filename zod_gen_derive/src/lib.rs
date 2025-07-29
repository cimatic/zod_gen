extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr};

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
                let var_name = LitStr::new(&v.ident.to_string(), v.ident.span());
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
