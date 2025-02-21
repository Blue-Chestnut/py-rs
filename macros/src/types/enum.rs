use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, ItemEnum, Variant};

use crate::{
    attr::{Attr, EnumAttr, FieldAttr, StructAttr, Tagged, VariantAttr},
    deps::Dependencies,
    types::{self, type_as, type_override},
    DerivedPY,
};

pub(crate) fn r#enum_def(s: &ItemEnum) -> syn::Result<DerivedPY> {
    let enum_attr: EnumAttr = EnumAttr::from_attrs(&s.attrs)?;

    enum_attr.assert_validity(s)?;

    let crate_rename = enum_attr.crate_rename();

    let name = match &enum_attr.rename {
        Some(existing) => existing.clone(),
        None => s.ident.to_string(),
    };

    if let Some(attr_type_override) = &enum_attr.type_override {
        return type_override::type_override_enum(&enum_attr, &name, attr_type_override);
    }

    if let Some(attr_type_as) = &enum_attr.type_as {
        return type_as::type_as_enum(&enum_attr, &name, attr_type_as);
    }

    if s.variants.is_empty() {
        return Ok(empty_enum(name, enum_attr));
    }

    let mut formatted_variants = Vec::new();
    let mut dependencies = Dependencies::new(crate_rename.clone());

    for variant in &s.variants {
        format_variant(
            &mut formatted_variants,
            &mut dependencies,
            &enum_attr,
            variant,
        )?;
    }
    let mut variants: Vec<String> = vec![];
    for variant in &s.variants {
        variants.push(variant.ident.to_string());
    }

    Ok(DerivedPY {
        crate_rename,
        inline: quote!([#(#formatted_variants),*].join(" | ")),
        inline_flattened: Some(quote!(
            format!("({})", [#(#formatted_variants),*].join(" | "))
        )),
        dependencies,
        docs: enum_attr.docs,
        export: enum_attr.export,
        export_to: enum_attr.export_to,
        py_name: name,
        is_enum: true,
        variants,
        concrete: enum_attr.concrete,
        bound: enum_attr.bound,
    })
}

fn format_variant(
    formatted_variants: &mut Vec<TokenStream>,
    dependencies: &mut Dependencies,
    enum_attr: &EnumAttr,
    variant: &Variant,
) -> syn::Result<()> {
    let crate_rename = enum_attr.crate_rename();

    // If `variant.fields` is not a `Fields::Named(_)` the `rename_all_fields`
    // attribute must be ignored to prevent a `rename_all` from getting to
    // the newtype, tuple or unit formatting, which would cause an error
    let variant_attr = VariantAttr::from_attrs(&variant.attrs)?;

    variant_attr.assert_validity(variant)?;

    if variant_attr.skip {
        return Ok(());
    }

    let untagged_variant = variant_attr.untagged;
    let name = match (variant_attr.rename.clone(), &enum_attr.rename_all) {
        (Some(rn), _) => rn,
        (None, None) => variant.ident.to_string(),
        (None, Some(rn)) => rn.apply(&variant.ident.to_string()),
    };

    let struct_attr = StructAttr::from_variant(enum_attr, &variant_attr, &variant.fields);
    let variant_type = types::type_def(
        &struct_attr,
        // In internally tagged enums, we can tag the struct
        &name,
        &variant.fields,
    )?;

    let variant_dependencies = variant_type.dependencies;
    let inline_type = variant_type.inline;

    let parsed_ty = match (&variant_attr.type_as, &variant_attr.type_override) {
        (Some(_), Some(_)) => syn_err_spanned!(variant; "`type` is not compatible with `as`"),
        (Some(ty), None) => {
            dependencies.push(ty);
            quote!(<#ty as #crate_rename::PY>::name())
        }
        (None, Some(ty)) => quote!(#ty.to_owned()),
        (None, None) => {
            dependencies.append(variant_dependencies);
            inline_type
        }
    };

    let formatted = match (untagged_variant, enum_attr.tagged()?) {
        (true, _) | (_, Tagged::Untagged) => quote!(#parsed_ty),
        (false, Tagged::Externally) => match &variant.fields {
            Fields::Unit => quote!(format!("\"{}\"", #name)),
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                let field = &unnamed.unnamed[0];
                let field_attr = FieldAttr::from_attrs(&field.attrs)?;

                field_attr.assert_validity(field)?;

                if field_attr.skip {
                    quote!(format!("\"{}\"", #name))
                } else {
                    quote!(format!("{{ \"{}\": {} }}", #name, #parsed_ty))
                }
            }
            _ => quote!(format!("{{ \"{}\": {} }}", #name, #parsed_ty)),
        },
        (false, Tagged::Adjacently { tag, content }) => match &variant.fields {
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                let field = &unnamed.unnamed[0];
                let field_attr = FieldAttr::from_attrs(&unnamed.unnamed[0].attrs)?;

                field_attr.assert_validity(field)?;

                if field_attr.skip {
                    quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #name))
                } else {
                    let ty = match field_attr.type_override {
                        Some(type_override) => quote!(#type_override),
                        None => {
                            let ty = field_attr.type_as(&field.ty);
                            quote!(<#ty as #crate_rename::PY>::name())
                        }
                    };
                    quote!(format!("{{ \"{}\": \"{}\", \"{}\": {} }}", #tag, #name, #content, #ty))
                }
            }
            Fields::Unit => quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #name)),
            _ => quote!(
                format!("{{ \"{}\": \"{}\", \"{}\": {} }}", #tag, #name, #content, #parsed_ty)
            ),
        },
        (false, Tagged::Internally { tag }) => match variant_type.inline_flattened {
            Some(_) => {
                quote! { #parsed_ty }
            }
            None => match &variant.fields {
                Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                    let field = &unnamed.unnamed[0];
                    let field_attr = FieldAttr::from_attrs(&unnamed.unnamed[0].attrs)?;

                    field_attr.assert_validity(field)?;

                    if field_attr.skip {
                        quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #name))
                    } else {
                        let ty = match field_attr.type_override {
                            Some(type_override) => quote! { #type_override },
                            None => {
                                let ty = field_attr.type_as(&field.ty);
                                quote!(<#ty as #crate_rename::PY>::name())
                            }
                        };

                        quote!(format!("{{ \"{}\": \"{}\" }} & {}", #tag, #name, #ty))
                    }
                }
                Fields::Unit => quote!(format!("{{ \"{}\": \"{}\" }}", #tag, #name)),
                _ => {
                    quote!(format!("{{ \"{}\": \"{}\" }} & {}", #tag, #name, #parsed_ty))
                }
            },
        },
    };

    formatted_variants.push(formatted);
    Ok(())
}

// bindings for an empty enum (`never` in PY)
fn empty_enum(name: impl Into<String>, enum_attr: EnumAttr) -> DerivedPY {
    let name = name.into();
    let crate_rename = enum_attr.crate_rename();
    DerivedPY {
        crate_rename: crate_rename.clone(),
        inline: quote!("None".to_owned()),
        docs: enum_attr.docs,
        inline_flattened: None,
        dependencies: Dependencies::new(crate_rename),
        export: enum_attr.export,
        export_to: enum_attr.export_to,
        py_name: name,
        concrete: enum_attr.concrete,
        bound: enum_attr.bound,
        is_enum: true,
        variants: vec![],
    }
}
