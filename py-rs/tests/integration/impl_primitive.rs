#[cfg(feature = "bigdecimal-impl")]
#[test]
fn impl_primitive_bigdecimal() {
    assert_eq!(
        <bigdecimal::BigDecimal as py_rs::PY>::name(),
        <String as py_rs::PY>::name()
    );
    assert_eq!(
        <bigdecimal::BigDecimal as py_rs::PY>::inline(),
        <String as py_rs::PY>::inline()
    )
}

#[cfg(feature = "smol_str-impl")]
#[test]
fn impl_primitive_smolstr() {
    assert_eq!(
        <smol_str::SmolStr as py_rs::PY>::name(),
        <String as py_rs::PY>::name()
    );
    assert_eq!(
        <smol_str::SmolStr as py_rs::PY>::inline(),
        <String as py_rs::PY>::inline()
    )
}

#[cfg(feature = "uuid-impl")]
#[test]
fn impl_primitive_uuid() {
    assert_eq!(
        <uuid::Uuid as py_rs::PY>::name(),
        <String as py_rs::PY>::name()
    );
    assert_eq!(
        <uuid::Uuid as py_rs::PY>::inline(),
        <String as py_rs::PY>::inline()
    )
}

#[cfg(feature = "url-impl")]
#[test]
fn impl_primitive_url() {
    assert_eq!(
        <url::Url as py_rs::PY>::name(),
        <String as py_rs::PY>::name()
    );
    assert_eq!(
        <url::Url as py_rs::PY>::inline(),
        <String as py_rs::PY>::inline()
    )
}

#[cfg(feature = "ordered-float-impl")]
#[test]
fn impl_primitive_order_float() {
    assert_eq!(
        <ordered_float::OrderedFloat<f64> as py_rs::PY>::name(),
        <f64 as py_rs::PY>::name()
    );
    assert_eq!(
        <ordered_float::OrderedFloat<f64> as py_rs::PY>::inline(),
        <f64 as py_rs::PY>::inline()
    );
    assert_eq!(
        <ordered_float::OrderedFloat<f32> as py_rs::PY>::name(),
        <f32 as py_rs::PY>::name()
    );
    assert_eq!(
        <ordered_float::OrderedFloat<f32> as py_rs::PY>::inline(),
        <f32 as py_rs::PY>::inline()
    )
}

#[cfg(feature = "bson-uuid-impl")]
#[test]
fn impl_primitive_bson_uuid() {
    assert_eq!(
        <bson::oid::ObjectId as py_rs::PY>::name(),
        <String as py_rs::PY>::name()
    );
    assert_eq!(
        <bson::oid::ObjectId as py_rs::PY>::inline(),
        <String as py_rs::PY>::inline()
    );
    assert_eq!(
        <bson::Uuid as py_rs::PY>::name(),
        <String as py_rs::PY>::name()
    );
    assert_eq!(
        <bson::Uuid as py_rs::PY>::inline(),
        <String as py_rs::PY>::inline()
    )
}

#[cfg(feature = "semver-impl")]
#[test]
fn impl_primitive_semver() {
    assert_eq!(
        <semver::Version as py_rs::PY>::name(),
        <String as py_rs::PY>::name()
    );
    assert_eq!(
        <semver::Version as py_rs::PY>::inline(),
        <String as py_rs::PY>::inline()
    )
}
