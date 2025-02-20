use quote::quote;
use syn::{Result, Type};

use crate::{
    attr::{ContainerAttr, EnumAttr, StructAttr},
    deps::Dependencies,
    DerivedPY,
};

pub(crate) fn type_as_struct(attr: &StructAttr, name: &str, type_as: &Type) -> Result<DerivedPY> {
    let crate_rename = attr.crate_rename();

    Ok(DerivedPY {
        crate_rename: crate_rename.clone(),
        inline: quote!(#type_as::inline()),
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies: Dependencies::new(crate_rename),
        export: attr.export,
        export_to: attr.export_to.clone(),
        py_name: name.to_owned(),
        is_enum: false,
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
    })
}

pub(crate) fn type_as_enum(attr: &EnumAttr, name: &str, type_as: &Type) -> Result<DerivedPY> {
    let crate_rename = attr.crate_rename();

    Ok(DerivedPY {
        crate_rename: crate_rename.clone(),
        inline: quote!(#type_as::inline()),
        is_enum: true,
        inline_flattened: None,
        docs: attr.docs.clone(),
        dependencies: Dependencies::new(crate_rename),
        export: attr.export,
        export_to: attr.export_to.clone(),
        py_name: name.to_owned(),
        concrete: attr.concrete.clone(),
        bound: attr.bound.clone(),
    })
}
