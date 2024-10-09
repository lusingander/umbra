use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{ParseStream, Parser},
    Error, Ident, ItemStruct, Result, Type,
};

pub(crate) fn opt_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    opt_parse
        .parse2(item)
        .unwrap_or_else(Error::into_compile_error)
}

fn opt_parse(input: ParseStream) -> Result<TokenStream> {
    let base_struct: ItemStruct = input.parse()?;

    let original_struct_block = build_original_struct_block(base_struct.clone());
    let optional_struct_block = build_optional_struct_block(base_struct.clone());

    let ts = quote! {
        #original_struct_block
        #optional_struct_block
    };
    Ok(ts)
}

fn build_original_struct_block(mut base_struct: ItemStruct) -> TokenStream {
    for field in &mut base_struct.fields {
        field.attrs.retain(|attr| !is_nested_attr(attr));
    }

    quote! {
        #base_struct
    }
}

fn build_optional_struct_block(base_struct: ItemStruct) -> TokenStream {
    let base_name = &base_struct.ident;
    let name = optional_struct_name(base_name);
    let fields: Vec<TokenStream> = base_struct
        .fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;
            let optional_type = if has_nested_attr(field) {
                match field_type {
                    Type::Path(type_path) => {
                        let type_ident = &type_path.path.segments.first().unwrap().ident;
                        let nested_struct_name = optional_struct_name(type_ident);
                        Type::Verbatim(quote! { Option<#nested_struct_name> })
                    }
                    _ => Type::Verbatim(quote! { Option<#field_type> }),
                }
            } else {
                Type::Verbatim(quote! { Option<#field_type> })
            };
            quote! {
                #field_name: #optional_type,
            }
        })
        .collect();

    let field_names: Vec<&Ident> = base_struct
        .fields
        .iter()
        .filter(|field| !has_nested_attr(field))
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    let nested_field_names: Vec<&Ident> = base_struct
        .fields
        .iter()
        .filter(|field| has_nested_attr(field))
        .map(|field| field.ident.as_ref().unwrap())
        .collect();

    quote! {
        struct #name {
            #(#fields)*
        }
        impl From<#name> for #base_name {
            fn from(optional: #name) -> Self {
                let mut base = Self::default();
                #(
                    if let Some(value) = optional.#field_names {
                        base.#field_names = value;
                    }
                )*
                #(
                    if let Some(value) = optional.#nested_field_names {
                        base.#nested_field_names = value.into();
                    }
                )*
                base
            }
        }
    }
}

fn has_nested_attr(field: &syn::Field) -> bool {
    field.attrs.iter().any(is_nested_attr)
}

fn is_nested_attr(attr: &syn::Attribute) -> bool {
    attr.path().is_ident("nested")
}

fn optional_struct_name(base_name: &Ident) -> Ident {
    Ident::new(&format!("Optional{}", base_name), base_name.span())
}
