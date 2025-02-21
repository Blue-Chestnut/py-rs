use quote::quote;
use syn::Result;

use crate::{
    attr::{ContainerAttr, EnumAttr, StructAttr},
    deps::Dependencies,
    DerivedPY, EnumDef,
};

pub(crate) fn type_override_struct(
    attr: &StructAttr,
    name: &str,
    type_override: &str,
) -> Result<DerivedPY> {
    let crate_rename = attr.crate_rename();

    Ok(DerivedPY {
        crate_rename: crate_rename.clone(),
        inline: quote!(#type_override.to_owned()),
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies: Dependencies::new(crate_rename),
        export: attr.export,
        export_to: attr.export_to.clone(),
        py_name: name.to_owned(),
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
        enum_def: None,
    })
}

pub(crate) fn type_override_enum(
    attr: &EnumAttr,
    name: &str,
    type_override: &str,
) -> Result<DerivedPY> {
    let crate_rename = attr.crate_rename();

    Ok(DerivedPY {
        crate_rename: crate_rename.clone(),
        inline: quote!(#type_override.to_owned()),
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies: Dependencies::new(crate_rename),
        export: attr.export,
        export_to: attr.export_to.clone(),
        py_name: name.to_owned(),
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
        enum_def: Some(EnumDef {
            ..Default::default()
        }), // TODO implement
    })
}
