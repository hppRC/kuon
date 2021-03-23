use std::collections::HashMap;

use heck::SnakeCase;
use proc_macro2::{Ident, TokenStream};
use proc_macro_utils::{
    count_generics_tyeps, extract_doc_attr, extract_generics_names, extract_inner_type,
    extract_struct_fields, is_int_type, is_option_type, is_vec_type,
};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, DeriveInput, Field, FieldsNamed,
    GenericParam, Generics, Token, TypeParam, TypeTuple,
};

#[proc_macro_derive(KuonRequest)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let func_doc = extract_doc_attr(&item.attrs);
    let struct_name = item.ident;
    let struct_name_snake = format_ident!("{}", struct_name.to_string().to_snake_case());
    let fields = extract_struct_fields(&item.data);
    let generics = item.generics;
    let generic_types_count = count_generics_tyeps(&generics);
    let generics_names = extract_generics_names(&generics);
    let unit = TypeTuple {
        paren_token: Default::default(),
        elems: Default::default(),
    };

    let units = (0..generic_types_count).into_iter().map(|_| {
        quote! {#unit}
    });

    let required_fields = extract_required_fields(fields);
    let optional_fields = extract_optional_fields(fields);
    let doc_comment_map: HashMap<String, Option<Attribute>> = required_fields
        .iter()
        .chain(optional_fields.iter())
        .filter_map(|&field| {
            Some((
                field.ident.clone()?.to_string(),
                extract_doc_attr(&field.attrs),
            ))
        })
        .collect();

    let required_inits = required_fields.iter().map(required_init);
    let optional_inits = optional_fields.iter().map(optional_init);
    let inits = required_inits.into_iter().chain(optional_inits);

    let impl_required_params = generics_names.iter().map(|name| {
        impl_required_fields(
            name,
            &struct_name,
            &generics,
            &required_fields,
            &optional_fields,
            &doc_comment_map,
        )
    });

    let optional_params_setter_stream_iter = optional_fields
        .iter()
        .map(|&field| optional_params_setter(&field, &doc_comment_map));
    let optional_params_to_hashmap_stream_iter = optional_fields.iter().map(to_hashmap_iflet);
    let generics_where_to_string_stream_iter = generics_names.iter().map(|name| {
        let generics_ident = format_ident!("{}", name);
        quote! {
            #generics_ident: ToString
        }
    });
    let to_hashmap_stream_iter = required_fields
        .iter()
        .map(|&field| {
            let ident = &field.ident;
            quote! {
                params.insert(stringify!(#ident), self.#ident.to_string());
            }
        })
        .chain(optional_params_to_hashmap_stream_iter.clone());

    let expanded = quote! {
        impl TwitterAPI {
            #func_doc
            pub fn #struct_name_snake(&self) -> #struct_name<#(#units),*> {
                #struct_name {
                    api: self,
                    #(#inits),*
                }
            }
        }

        #(#impl_required_params)*

        impl#generics #struct_name#generics {
            fn optional_params_to_hashmap(&self) -> std::collections::HashMap<&str, String> {
                let mut params = std::collections::HashMap::new();
                #(#optional_params_to_hashmap_stream_iter)*
                params
            }

            #(#optional_params_setter_stream_iter)*
        }

        impl#generics #struct_name#generics where
            #(#generics_where_to_string_stream_iter),*
        {
            fn to_hashmap(&self) -> std::collections::HashMap<&str, String> {
                let mut params = std::collections::HashMap::new();
                #(#to_hashmap_stream_iter)*
                params
            }
        }
    };
    // println!("{}", expanded.clone().to_string());
    proc_macro::TokenStream::from(expanded)
}

fn extract_optional_fields(fields: &FieldsNamed) -> Vec<&Field> {
    fields
        .named
        .iter()
        .filter(|&field| is_option_type(&field.ty))
        .collect()
}

fn extract_required_fields(fields: &FieldsNamed) -> Vec<&Field> {
    fields
        .named
        .iter()
        .filter(|&field| {
            if let Some(ident) = &field.ident {
                !is_option_type(&field.ty) && ident != "api"
            } else {
                false
            }
        })
        .collect()
}

fn to_hashmap_iflet(&field: &&Field) -> TokenStream {
    let ty = &field.ty;
    let ident = &field.ident;
    if let syn::GenericArgument::Type(inner_ty) = extract_inner_type(ty) {
        if is_vec_type(inner_ty) {
            quote! {
                if let Some(ref #ident) = self.#ident {
                    params.insert(
                        stringify!(#ident),
                        #ident.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",")
                    );
                }
            }
        } else {
            quote! {
                if let Some(ref #ident) = self.#ident {
                    params.insert(stringify!(#ident), #ident.to_string());
                }
            }
        }
    } else {
        quote! {}
    }
}

fn required_init(&field: &&Field) -> TokenStream {
    // dbg!(field.clone());
    let ident = &field.ident;
    quote! {
        #ident: ()
    }
}

fn optional_init(&field: &&Field) -> TokenStream {
    let ident = &field.ident;
    quote! {
        #ident: None
    }
}

fn optional_params_setter(
    &field: &&Field,
    doc_comment_map: &HashMap<String, Option<Attribute>>,
) -> TokenStream {
    let ty = &field.ty;
    let ident = &field.ident;
    let doc = if let Some(ident) = ident {
        doc_comment_map.get(&ident.to_string())
    } else {
        None
    };
    if let syn::GenericArgument::Type(inner_ty) = extract_inner_type(ty) {
        if is_int_type(inner_ty) {
            quote! {
                #doc
                pub fn #ident(&mut self, #ident: #inner_ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        } else {
            quote! {
                #doc
                pub fn #ident(&mut self, #ident: impl Into<#inner_ty>) -> &mut Self {
                    self.#ident = Some(#ident.into());
                    self
                }
            }
        }
    } else {
        quote! {}
    }
}

fn impl_required_fields(
    name: &str,
    struct_name: &Ident,
    generics: &Generics,
    required_fields: &[&Field],
    optional_fields: &[&Field],
    doc_comment_map: &HashMap<String, Option<Attribute>>,
) -> TokenStream {
    let first_half: Punctuated<&GenericParam, Token![,]> = generics
        .params
        .iter()
        .take_while(|&param| match param {
            syn::GenericParam::Type(TypeParam { ident, .. }) => ident != name,
            _ => true,
        })
        .collect();
    let latter_half: Punctuated<&GenericParam, Token![,]> = generics
        .params
        .iter()
        .skip_while(|&param| match param {
            syn::GenericParam::Type(TypeParam { ident, .. }) => ident != name,
            _ => true,
        })
        .skip(1)
        .collect();
    let generics_ident = format_ident!("{}", name);
    let ident = format_ident!("{}", name.to_snake_case());
    let doc = doc_comment_map.get(&ident.to_string());

    let fields_settings = required_fields
        .iter()
        .filter(|&field| {
            if let Some(req_ident) = &field.ident {
                req_ident != &ident
            } else {
                true
            }
        })
        .chain(optional_fields.iter())
        .map(|&field| {
            let ident = &field.ident;
            quote! {
                #ident: self.#ident
            }
        });

    let tmp = quote! {
        #doc
        pub fn #ident<#generics_ident>(self, #ident: #generics_ident) -> #struct_name#generics
        where
        #generics_ident: ToString,
        {
            #struct_name {
                api: self.api,
                #ident,
                #(#fields_settings),*
            }
        }
    };

    if !latter_half.is_empty() {
        quote! {
            impl<#first_half, #latter_half> #struct_name<#first_half, (), #latter_half> {
                #tmp
            }
        }
    } else {
        quote! {
            impl<#first_half> #struct_name<#first_half, ()> {
                #tmp
            }
        }
    }
}
