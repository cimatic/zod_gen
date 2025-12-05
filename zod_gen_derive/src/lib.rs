extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Attribute, Data, DeriveInput, Fields, LitStr};

/// Helper function to extract the serde rename value from attributes
///
/// Returns the renamed value if #[serde(rename = "...")] is found, otherwise
/// None
fn find_serde_rename_from_attrs(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
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
                        return Some(rename_value.to_string());
                    }
                }
            }
        }
    }
    None
}

fn find_serde_tag_from_attrs(attrs: &[Attribute]) -> String {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            let attr_str = attr.to_token_stream().to_string();
            if let Some(tag_start) = attr_str.find("tag") {
                let tag_part = &attr_str[tag_start..];
                if let Some(quote_start) = tag_part.find('"') {
                    if let Some(quote_end) = tag_part[quote_start + 1..].find('"') {
                        let tag_value = &tag_part[quote_start + 1..quote_start + 1 + quote_end];
                        return tag_value.to_string();
                    }
                }
            }
        }
    }
    "enumField".to_string()
}

/// Helper function to extract the serde rename value from variant attributes
///
/// Returns the renamed value if #[serde(rename = "...")] is found,
/// otherwise returns the variant name as a string.
fn extract_serde_rename_variant(variant: &syn::Variant) -> String {
    if let Some(rename_value) = find_serde_rename_from_attrs(&variant.attrs) {
        rename_value
    } else {
        // Fallback to variant name if no serde rename found
        variant.ident.to_string()
    }
}

/// Derive macro for ZodSchema
#[proc_macro_derive(ZodSchema)]
pub fn derive_zod_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_span = name.span();

    let expanded = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => {
                let fields = fields_named.named.iter().map(|f| {
                    let ident = f.ident.as_ref().unwrap();
                    let ident_name =
                        find_serde_rename_from_attrs(&f.attrs).unwrap_or_else(|| ident.to_string());
                    let field_name = LitStr::new(&ident_name, ident.span());
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
            let all_unit_variants = data_enum
                .variants
                .iter()
                .all(|v| matches!(v.fields, Fields::Unit));

            let enum_schema_tokens = if all_unit_variants {
                // If all enum variants are Unit variants, the schema will be a union of literals.
                let literal_variants: Vec<proc_macro2::TokenStream> = data_enum
                    .variants
                    .iter()
                    .map(|v| {
                        let renamed_value = extract_serde_rename_variant(v);
                        let variant_literal_val = LitStr::new(&renamed_value, v.ident.span());
                        quote! { zod_gen::zod_literal(#variant_literal_val) }
                    })
                    .collect();

                quote! {
                    let __owned_literals: Vec<String> = vec![
                        #(#literal_variants.to_string()),*
                    ];
                    let __literal_refs: Vec<&str> = __owned_literals
                        .iter()
                        .map(|s| s.as_str())
                        .collect();
                    zod_gen::zod_union(&__literal_refs)
                }
            } else {
                // If at least one variant is not a Unit variant. Generates a discriminated union of objects with a dedicated key.
                let tag_field = find_serde_tag_from_attrs(&input.attrs);
                let tag_key_lit = LitStr::new(&tag_field, name_span);

                let variant_schema_tokens: Vec<proc_macro2::TokenStream> = data_enum.variants.iter().map(|v| {
                           let renamed_value = extract_serde_rename_variant(v);
                           let variant_literal_val = LitStr::new(&renamed_value, v.ident.span());

                           match &v.fields {
                               Fields::Unit => {
                                   quote! {
                                       {
                                           let __literal_schema = zod_gen::zod_literal(#variant_literal_val);
                                           zod_gen::zod_object(&[
                                               (#tag_key_lit, __literal_schema.as_str())
                                           ])
                                       }
                                   }
                               }
                               Fields::Unnamed(fields) => {
                                   if fields.unnamed.len() == 1 {
                                       let field_type = &fields.unnamed.first().unwrap().ty;
                                       quote! {
                                           {
                                               let __literal_schema = zod_gen::zod_literal(#variant_literal_val);
                                               zod_gen::zod_object(&[
                                                   (#tag_key_lit, __literal_schema.as_str()),
                                                   ("data", &<#field_type as zod_gen::ZodSchema>::zod_schema())
                                               ])
                                           }
                                       }
                                   } else {
                                       let inner_fields = fields.unnamed.iter().enumerate().map(|(i, f)| {
                                           let field_name = LitStr::new(&i.to_string(), f.span());
                                           let field_type = &f.ty;
                                           quote! { (#field_name, &<#field_type as zod_gen::ZodSchema>::zod_schema()) }
                                       });
                                       quote! {
                                           {
                                               let __literal_schema = zod_gen::zod_literal(#variant_literal_val);
                                               let __inner_data_object = zod_gen::zod_object(&[#(#inner_fields),*]);
                                               zod_gen::zod_object(&[
                                                   (#tag_key_lit, __literal_schema.as_str()),
                                                   ("data", __inner_data_object.as_str())
                                               ])
                                           }
                                       }
                                   }
                               }
                               Fields::Named(fields) => {
                                   let inner_fields = fields.named.iter().map(|f| {
                                       let ident = f.ident.as_ref().unwrap();
                                       let ident_name = find_serde_rename_from_attrs(&f.attrs)
                                           .unwrap_or_else(|| ident.to_string());
                                       let field_name = LitStr::new(&ident_name, ident.span());
                                       let ty = &f.ty;
                                       quote! { (#field_name, &<#ty as zod_gen::ZodSchema>::zod_schema()) }
                                   });
                                   quote! {
                                       {
                                           let __literal_schema = zod_gen::zod_literal(#variant_literal_val);
                                           zod_gen::zod_object(&[
                                               (#tag_key_lit, __literal_schema.as_str()),
                                               #(#inner_fields),*
                                           ])
                                       }
                                   }
                               }
                           }
                       }).collect();

                quote! {
                    let __owned_schemas: Vec<String> = vec![
                        #(#variant_schema_tokens.to_string()),*
                    ];
                    let __schema_refs: Vec<&str> = __owned_schemas
                        .iter()
                        .map(|s| s.as_str())
                        .collect();
                    zod_gen::zod_union(&__schema_refs)
                }
            };

            quote! {
                impl zod_gen::ZodSchema for #name {
                    fn zod_schema() -> String {
                        #enum_schema_tokens
                    }
                }
            }
        }
        _ => panic!("ZodSchema derive only supports structs and enums"),
    };

    TokenStream::from(expanded)
}
