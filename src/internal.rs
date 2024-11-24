use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    Ident, ItemStruct, Result, Type,
};

pub(crate) fn opt_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let base_struct = parse2(item);
    let attributes = parse2(attr);

    match (base_struct, attributes) {
        (Ok(base_struct), Ok(attributes)) => build(base_struct, attributes),
        (Err(e), _) => e.to_compile_error(),
        (_, Err(e)) => e.to_compile_error(),
    }
}

#[derive(Clone)]
struct Attributes {
    derives: Vec<String>,
    prefix: String,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            derives: vec![],
            prefix: "Optional".into(),
        }
    }
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(Self::default());
        }

        let mut attributes = Self::default();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;

            if ident == "derives" {
                let _: syn::Token![=] = input.parse()?;

                let content;
                syn::bracketed!(content in input);
                attributes.derives =
                    Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated(&content)?
                        .into_iter()
                        .map(|lit| lit.value())
                        .collect();
            } else if ident == "prefix" {
                let _: syn::Token![=] = input.parse()?;
                let lit: syn::LitStr = input.parse()?;
                attributes.prefix = lit.value();
            }

            if input.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        }

        Ok(attributes)
    }
}

fn build(base_struct: ItemStruct, attributes: Attributes) -> TokenStream {
    let original_struct_block = build_original_struct_block(base_struct.clone());
    let optional_struct_block =
        build_optional_struct_block(base_struct.clone(), attributes.clone());

    quote! {
        #original_struct_block
        #optional_struct_block
    }
}

fn build_original_struct_block(mut base_struct: ItemStruct) -> TokenStream {
    for field in &mut base_struct.fields {
        field.attrs.retain(|attr| !is_nested_attr(attr));
    }

    quote! {
        #base_struct
    }
}

fn build_optional_struct_block(base_struct: ItemStruct, attributes: Attributes) -> TokenStream {
    let derives = attributes
        .derives
        .iter()
        .map(|s| Ident::new(s, base_struct.ident.span()));

    let base_name = &base_struct.ident;
    let name = optional_struct_name(base_name, &attributes);
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
                        let nested_struct_name = optional_struct_name(type_ident, &attributes);
                        Type::Verbatim(quote! { Option<#nested_struct_name> })
                    }
                    _ => Type::Verbatim(quote! { Option<#field_type> }),
                }
            } else if is_option_type(field_type) {
                Type::Verbatim(quote! { #field_type })
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
        .filter(|field| !has_nested_attr(field) && !is_option_type(&field.ty))
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    let option_field_names: Vec<&Ident> = base_struct
        .fields
        .iter()
        .filter(|field| is_option_type(&field.ty))
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    let nested_field_names: Vec<&Ident> = base_struct
        .fields
        .iter()
        .filter(|field| has_nested_attr(field))
        .map(|field| field.ident.as_ref().unwrap())
        .collect();

    quote! {
        #[derive(#(#derives),*)]
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
                    if let Some(value) = optional.#option_field_names {
                        base.#option_field_names = Some(value);
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

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        let type_ident = &type_path.path.segments.first().unwrap().ident;
        type_ident == "Option"
    } else {
        false
    }
}

fn optional_struct_name(base_name: &Ident, attributes: &Attributes) -> Ident {
    Ident::new(
        &format!("{}{}", attributes.prefix, base_name),
        base_name.span(),
    )
}
