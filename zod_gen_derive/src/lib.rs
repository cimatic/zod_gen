extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, LitStr};

#[derive(Clone)]
enum EnumRepresentation {
    ExternallyTagged,
    InternallyTagged { tag: String },
    AdjacentlyTagged { tag: String, content: String },
    Untagged,
}

fn find_serde_rename_from_attrs(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            let attr_str = quote!(#attr).to_string();
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

fn extract_serde_rename_variant(variant: &syn::Variant) -> String {
    if let Some(rename_value) = find_serde_rename_from_attrs(&variant.attrs) {
        rename_value
    } else {
        variant.ident.to_string()
    }
}

fn find_serde_tag_from_attrs(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            let attr_str = quote!(#attr).to_string();
            if let Some(tag_start) = attr_str.find("tag") {
                let tag_part = &attr_str[tag_start..];
                if let Some(quote_start) = tag_part.find('"') {
                    if let Some(quote_end) = tag_part[quote_start + 1..].find('"') {
                        let tag_value = &tag_part[quote_start + 1..quote_start + 1 + quote_end];
                        return Some(tag_value.to_string());
                    }
                }
            }
        }
    }
    None
}

fn find_serde_content_from_attrs(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            let attr_str = quote!(#attr).to_string();
            if let Some(content_start) = attr_str.find("content") {
                let content_part = &attr_str[content_start..];
                if let Some(quote_start) = content_part.find('"') {
                    if let Some(quote_end) = content_part[quote_start + 1..].find('"') {
                        let content_value =
                            &content_part[quote_start + 1..quote_start + 1 + quote_end];
                        return Some(content_value.to_string());
                    }
                }
            }
        }
    }
    None
}

fn has_serde_untagged(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            let attr_str = quote!(#attr).to_string();
            if attr_str.contains("untagged") {
                return true;
            }
        }
    }
    false
}

fn parse_enum_serde_attrs(attrs: &[Attribute]) -> Result<EnumRepresentation, String> {
    let has_untagged = has_serde_untagged(attrs);
    let tag = find_serde_tag_from_attrs(attrs);
    let content = find_serde_content_from_attrs(attrs);

    if has_untagged {
        if tag.is_some() || content.is_some() {
            return Err("enum cannot be both untagged and internally tagged".to_string());
        }
        return Ok(EnumRepresentation::Untagged);
    }

    if let Some(t) = tag {
        if let Some(c) = content {
            return Ok(EnumRepresentation::AdjacentlyTagged { tag: t, content: c });
        } else {
            return Ok(EnumRepresentation::InternallyTagged { tag: t });
        }
    }

    Ok(EnumRepresentation::ExternallyTagged)
}

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
            _ => {
                return syn::Error::new(
                    name_span,
                    "ZodSchema derive only supports structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        Data::Enum(data_enum) => {
            let representation = parse_enum_serde_attrs(&input.attrs)
                .expect("Failed to parse serde enum attributes");

            match representation {
                EnumRepresentation::ExternallyTagged => {
                    let all_unit = data_enum
                        .variants
                        .iter()
                        .all(|v| matches!(v.fields, Fields::Unit));

                    if all_unit {
                        let literal_variants: Vec<proc_macro2::TokenStream> = data_enum
                            .variants
                            .iter()
                            .map(|v| {
                                let renamed = extract_serde_rename_variant(v);
                                let lit = LitStr::new(&renamed, v.ident.span());
                                quote! { zod_gen::zod_literal(#lit) }
                            })
                            .collect();
                        quote! {
                            impl zod_gen::ZodSchema for #name {
                                fn zod_schema() -> String {
                                    let owned: Vec<String> = vec![#(#literal_variants.to_string()),*];
                                    let refs: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
                                    zod_gen::zod_union(&refs)
                                }
                            }
                        }
                    } else {
                        let variant_schemas: Vec<proc_macro2::TokenStream> = data_enum.variants.iter().map(|v| {
                            let renamed = extract_serde_rename_variant(v);
                            let var_lit = LitStr::new(&renamed, v.ident.span());

                            match &v.fields {
                                Fields::Unit => {
                                    quote! {
                                        zod_gen::zod_literal(#var_lit)
                                    }
                                }
                                Fields::Unnamed(fields) => {
                                    if fields.unnamed.len() == 1 {
                                        let field_ty = &fields.unnamed.first().unwrap().ty;
                                        quote! {
                                            {
                                                let lit = zod_gen::zod_literal(#var_lit);
                                                let payload = <#field_ty as zod_gen::ZodSchema>::zod_schema();
                                                zod_gen::zod_object(&[(#var_lit, payload.as_str())])
                                            }
                                        }
                                } else {
                                    let inner_fields: Vec<proc_macro2::TokenStream> = fields.unnamed.iter().map(|f| {
                                        let field_ty = &f.ty;
                                        quote! { <#field_ty as zod_gen::ZodSchema>::zod_schema().as_str() }
                                    }).collect();
                                        quote! {
                                            {
                                                let lit = zod_gen::zod_literal(#var_lit);
                                                let inner = zod_gen::zod_tuple(&[#(#inner_fields),*]);
                                                zod_gen::zod_object(&[(#var_lit, inner.as_str())])
                                            }
                                        }
                                    }
                                }
                                Fields::Named(fields) => {
                                    let inner_fields: Vec<proc_macro2::TokenStream> = fields.named.iter().map(|f| {
                                        let ident = f.ident.as_ref().unwrap();
                                        let field_name = find_serde_rename_from_attrs(&f.attrs)
                                            .unwrap_or_else(|| ident.to_string());
                                        let name_lit = LitStr::new(&field_name, ident.span());
                                        let field_ty = &f.ty;
                                        quote! { (#name_lit, <#field_ty as zod_gen::ZodSchema>::zod_schema().as_str()) }
                                    }).collect();
                                    quote! {
                                        {
                                            let lit = zod_gen::zod_literal(#var_lit);
                                            let inner = zod_gen::zod_object(&[#(#inner_fields),*]);
                                            zod_gen::zod_object(&[(#var_lit, inner.as_str())])
                                        }
                                    }
                                }
                            }
                        }).collect();

                        quote! {
                            impl zod_gen::ZodSchema for #name {
                                fn zod_schema() -> String {
                                    let owned: Vec<String> = vec![#(#variant_schemas.to_string()),*];
                                    let refs: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
                                    zod_gen::zod_union(&refs)
                                }
                            }
                        }
                    }
                }
                EnumRepresentation::InternallyTagged { tag } => {
                    let tag_lit = LitStr::new(&tag, name_span);

                    for variant in &data_enum.variants {
                        match &variant.fields {
                            Fields::Unnamed(fields) => {
                                if fields.unnamed.len() != 1 {
                                    return syn::Error::new_spanned(
                                        variant,
                                        "#[serde(tag = \"...\")] cannot be used with tuple variants",
                                    )
                                    .to_compile_error()
                                    .into();
                                }
                            }
                            Fields::Unit => {}
                            Fields::Named(_) => {}
                        }
                    }

                    let variant_schemas: Vec<proc_macro2::TokenStream> = data_enum.variants.iter().map(|v| {
                        let renamed = extract_serde_rename_variant(v);
                        let var_lit = LitStr::new(&renamed, v.ident.span());

                        match &v.fields {
                            Fields::Unit => {
                                quote! {
                                    zod_gen::zod_object(&[(#tag_lit, zod_gen::zod_literal(#var_lit).as_str())])
                                }
                            }
                            Fields::Unnamed(fields) => {
                                if fields.unnamed.len() == 1 {
                                    let field_ty = &fields.unnamed.first().unwrap().ty;
                                    let tag_obj = quote! {
                                        zod_gen::zod_object(&[(#tag_lit, zod_gen::zod_literal(#var_lit).as_str())])
                                    };
                                    quote! {
                                        {
                                            let tag_obj = #tag_obj;
                                            let payload = <#field_ty as zod_gen::ZodSchema>::zod_schema();
                                            zod_gen::zod_intersection(&tag_obj, &payload)
                                        }
                                    }
                                } else {
                                    #[allow(clippy::useless_conversion, clippy::needless_return)]
                                    {
                                        return syn::Error::new_spanned(
                                            v,
                                            "#[serde(tag = \"...\")] cannot be used with tuple variants",
                                        )
                                        .to_compile_error()
                                        .into();
                                    }
                                }
                            }
                            Fields::Named(fields) => {
                                let inner_fields: Vec<proc_macro2::TokenStream> = fields.named.iter().map(|f| {
                                    let ident = f.ident.as_ref().unwrap();
                                    let field_name = find_serde_rename_from_attrs(&f.attrs)
                                        .unwrap_or_else(|| ident.to_string());
                                    let name_lit = LitStr::new(&field_name, ident.span());
                                    let field_ty = &f.ty;
                                    quote! { (#name_lit, <#field_ty as zod_gen::ZodSchema>::zod_schema().as_str()) }
                                }).collect();
                                quote! {
                                    {
                                        let lit = zod_gen::zod_literal(#var_lit);
                                        zod_gen::zod_object(&[(#tag_lit, lit.as_str()), #(#inner_fields),*])
                                    }
                                }
                            }
                        }
                    }).collect();

                    quote! {
                        impl zod_gen::ZodSchema for #name {
                            fn zod_schema() -> String {
                                let owned: Vec<String> = vec![#(#variant_schemas.to_string()),*];
                                let refs: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
                                zod_gen::zod_discriminated_union(#tag_lit, &refs)
                            }
                        }
                    }
                }
                EnumRepresentation::AdjacentlyTagged { tag, content } => {
                    let tag_lit = LitStr::new(&tag, name_span);
                    let content_lit = LitStr::new(&content, name_span);

                    let variant_schemas: Vec<proc_macro2::TokenStream> = data_enum.variants.iter().map(|v| {
                        let renamed = extract_serde_rename_variant(v);
                        let var_lit = LitStr::new(&renamed, v.ident.span());

                        match &v.fields {
                            Fields::Unit => {
                                quote! {
                                    zod_gen::zod_object(&[(#tag_lit, zod_gen::zod_literal(#var_lit).as_str())])
                                }
                            }
                            Fields::Unnamed(fields) => {
                                if fields.unnamed.len() == 1 {
                                    let field_ty = &fields.unnamed.first().unwrap().ty;
                                    quote! {
                                        {
                                            let lit = zod_gen::zod_literal(#var_lit);
                                            let payload = <#field_ty as zod_gen::ZodSchema>::zod_schema();
                                            zod_gen::zod_object(&[(#tag_lit, lit.as_str()), (#content_lit, payload.as_str())])
                                        }
                                    }
                                } else {
                                    let inner_fields: Vec<proc_macro2::TokenStream> = fields.unnamed.iter().map(|f| {
                                        let field_ty = &f.ty;
                                        quote! { <#field_ty as zod_gen::ZodSchema>::zod_schema().as_str() }
                                    }).collect();
                                    quote! {
                                        {
                                            let lit = zod_gen::zod_literal(#var_lit);
                                            let payload = zod_gen::zod_tuple(&[#(#inner_fields),*]);
                                            zod_gen::zod_object(&[(#tag_lit, lit.as_str()), (#content_lit, payload.as_str())])
                                        }
                                    }
                                }
                            }
                            Fields::Named(fields) => {
                                let inner_fields: Vec<proc_macro2::TokenStream> = fields.named.iter().map(|f| {
                                    let ident = f.ident.as_ref().unwrap();
                                    let field_name = find_serde_rename_from_attrs(&f.attrs)
                                        .unwrap_or_else(|| ident.to_string());
                                    let name_lit = LitStr::new(&field_name, ident.span());
                                    let field_ty = &f.ty;
                                    quote! { (#name_lit, <#field_ty as zod_gen::ZodSchema>::zod_schema().as_str()) }
                                }).collect();
                                quote! {
                                    {
                                        let lit = zod_gen::zod_literal(#var_lit);
                                        let payload = zod_gen::zod_object(&[#(#inner_fields),*]);
                                        zod_gen::zod_object(&[(#tag_lit, lit.as_str()), (#content_lit, payload.as_str())])
                                    }
                                }
                            }
                        }
                    }).collect();

                    quote! {
                        impl zod_gen::ZodSchema for #name {
                            fn zod_schema() -> String {
                                let owned: Vec<String> = vec![#(#variant_schemas.to_string()),*];
                                let refs: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
                                zod_gen::zod_discriminated_union(#tag_lit, &refs)
                            }
                        }
                    }
                }
                EnumRepresentation::Untagged => {
                    let variant_schemas: Vec<proc_macro2::TokenStream> = data_enum.variants.iter().map(|v| {
                        match &v.fields {
                            Fields::Unit => {
                                quote! { zod_gen::zod_null() }
                            }
                            Fields::Unnamed(fields) => {
                                if fields.unnamed.len() == 1 {
                                    let field_ty = &fields.unnamed.first().unwrap().ty;
                                    quote! {
                                        <#field_ty as zod_gen::ZodSchema>::zod_schema()
                                    }
                                } else {
                                    let inner_fields: Vec<proc_macro2::TokenStream> = fields.unnamed.iter().map(|f| {
                                        let field_ty = &f.ty;
                                        quote! { <#field_ty as zod_gen::ZodSchema>::zod_schema().as_str() }
                                    }).collect();
                                    quote! {
                                        zod_gen::zod_tuple(&[#(#inner_fields),*])
                                    }
                                }
                            }
                            Fields::Named(fields) => {
                                let inner_fields: Vec<proc_macro2::TokenStream> = fields.named.iter().map(|f| {
                                    let ident = f.ident.as_ref().unwrap();
                                    let field_name = find_serde_rename_from_attrs(&f.attrs)
                                        .unwrap_or_else(|| ident.to_string());
                                    let name_lit = LitStr::new(&field_name, ident.span());
                                    let field_ty = &f.ty;
                                    quote! { (#name_lit, <#field_ty as zod_gen::ZodSchema>::zod_schema().as_str()) }
                                }).collect();
                                quote! {
                                    zod_gen::zod_object(&[#(#inner_fields),*])
                                }
                            }
                        }
                    }).collect();

                    quote! {
                        impl zod_gen::ZodSchema for #name {
                            fn zod_schema() -> String {
                                let owned: Vec<String> = vec![#(#variant_schemas.to_string()),*];
                                let refs: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
                                zod_gen::zod_union(&refs)
                            }
                        }
                    }
                }
            }
        }
        _ => {
            return syn::Error::new(
                name_span,
                "ZodSchema derive only supports structs and enums",
            )
            .to_compile_error()
            .into();
        }
    };

    TokenStream::from(expanded)
}
