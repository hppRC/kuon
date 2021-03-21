use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Error, Fields, FieldsNamed, GenericArgument,
    Generics, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Path, PathArguments, PathSegment,
    Type, TypePath,
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
    match ty {
        &Type::Path(TypePath {
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
