use proc_macro2::TokenStream;
use proc_macro_utils::{extract_inner_type, extract_struct_fields, is_option_type, is_vec_type};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, FieldsNamed};

#[proc_macro_derive(KuonRequest)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let struct_name = item.ident;
    let fields = extract_struct_fields(&item.data);
    let generics = item.generics;

    let required_fields: Vec<&Field> = fields
        .named
        .iter()
        .filter(|&field| !is_option_type(&field.ty) && *field.ident.unwrap().to_string() != "api")
        .collect();
    let optional_fields: Vec<&Field> = fields
        .named
        .iter()
        .filter(|&field| is_option_type(&field.ty))
        .collect();

    let required_params_setter_stream_iter = required_fields.iter().map(f);
    let optional_params_setter_stream_iter = optional_fields.iter().map(optional_params_setter);
    let to_hashmap_stream_iter = optional_fields.iter().map(to_hashmap_iflet);

    let expanded = quote! {
        impl#generics #struct_name#generics {
            fn to_hashmap(&self) -> std::collections::HashMap<&str, String> {
                let mut params = std::collections::HashMap::new();
                #(#to_hashmap_stream_iter)*
                params
            }

            #(#optional_params_setter_stream_iter)*
        }
    };
    proc_macro::TokenStream::from(expanded)
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

fn optional_params_setter(&field: &&Field) -> TokenStream {
    let ty = &field.ty;
    let ident = &field.ident;
    if let syn::GenericArgument::Type(inner_ty) = extract_inner_type(ty) {
        quote! {
            pub fn #ident(&mut self, #ident: impl Into<#inner_ty>) -> &mut Self {
                self.#ident = Some(#ident.into());
                self
            }
        }
    } else {
        quote! {}
    }
}

fn f(&field: &&Field) -> TokenStream {
    todo!()
}
