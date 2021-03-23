use if_chain::if_chain;
use syn::{
    Attribute, Data, Fields, FieldsNamed, GenericArgument, Generics, Meta, MetaNameValue, Path,
    PathArguments, PathSegment, Type, TypeParam, TypePath,
};

pub fn is_option_type(ty: &Type) -> bool {
    match last_path_segment(&ty) {
        std::option::Option::Some(path_seg) => path_seg.ident == "Option",
        std::option::Option::None => false,
    }
}

pub fn is_vec_type(ty: &Type) -> bool {
    match last_path_segment(&ty) {
        std::option::Option::Some(path_seg) => path_seg.ident == "Vec",
        std::option::Option::None => false,
    }
}

pub fn is_int_type(ty: &Type) -> bool {
    match last_path_segment(&ty) {
        std::option::Option::Some(path_seg) => matches!(
            path_seg.ident.to_string().as_str(),
            "u128" | "u64" | "u32" | "u8" | "i128" | "i64" | "i32" | "i8"
        ),
        std::option::Option::None => false,
    }
}

pub fn extract_inner_type(ty: &Type) -> &GenericArgument {
    match last_path_segment(&ty) {
        std::option::Option::Some(PathSegment {
            ident: _,
            arguments: PathArguments::AngleBracketed(ref gen_arg),
        }) => gen_arg.args.first(),
        _ => std::option::Option::None,
    }
    .expect("invalid option type")
}

pub fn last_path_segment(ty: &Type) -> std::option::Option<&PathSegment> {
    match *ty {
        Type::Path(TypePath {
            qself: std::option::Option::None,
            path:
                Path {
                    segments: ref seg,
                    leading_colon: _,
                },
        }) => seg.last(),
        _ => std::option::Option::None,
    }
}

pub fn extract_struct_fields(data: &Data) -> &FieldsNamed {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields,
            _ => panic!("invalid fields"),
        },
        _ => panic!("invalid data"),
    }
}

pub fn count_generics_types(Generics { params, .. }: &Generics) -> u64 {
    params
        .iter()
        .map(|param| match param {
            syn::GenericParam::Type(_) => 1,
            _ => 0,
        })
        .sum()
}

pub fn extract_generics_names(Generics { params, .. }: &Generics) -> Vec<String> {
    params
        .iter()
        .filter_map(|param| match param {
            syn::GenericParam::Type(TypeParam { ident, .. }) => Some(ident.to_string()),
            _ => None,
        })
        .collect()
}

pub fn extract_doc_attr(attrs: &[Attribute]) -> std::option::Option<Attribute> {
    attrs.iter().find_map(|attr| {
        if_chain! {
            if let Ok(
                Meta::NameValue(
                    MetaNameValue { path, ..}
                )
            ) = attr.parse_meta();
            if path.get_ident()? == "doc";
            then {
                std::option::Option::Some(attr.clone())
            }
            else {
                std::option::Option::None
            }
        }
    })
}
