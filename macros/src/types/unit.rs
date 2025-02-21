use quote::quote;
use syn::Result;

use crate::{
    attr::{ContainerAttr, StructAttr},
    deps::Dependencies,
    DerivedPY,
};

pub(crate) fn empty_object(attr: &StructAttr, name: &str) -> Result<DerivedPY> {
    check_attributes(attr)?;
    let crate_rename = attr.crate_rename();

    Ok(DerivedPY {
        crate_rename: crate_rename.clone(),
        inline: quote!("Record<string, never>".to_owned()),
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

pub(crate) fn empty_array(attr: &StructAttr, name: &str) -> Result<DerivedPY> {
    check_attributes(attr)?;
    let crate_rename = attr.crate_rename();

    Ok(DerivedPY {
        crate_rename: crate_rename.clone(),
        inline: quote!("never[]".to_owned()),
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

pub(crate) fn null(attr: &StructAttr, name: &str) -> Result<DerivedPY> {
    check_attributes(attr)?;
    let crate_rename = attr.crate_rename();

    Ok(DerivedPY {
        crate_rename: crate_rename.clone(),
        inline: quote!("None".to_owned()),
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

fn check_attributes(attr: &StructAttr) -> Result<()> {
    if attr.rename_all.is_some() {
        syn_err!("`rename_all` is not applicable to unit structs");
    }

    if attr.tag.is_some() {
        syn_err!("`tag` is not applicable to unit structs");
    }

    Ok(())
}
