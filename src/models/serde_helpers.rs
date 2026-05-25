//! Internal helpers for serde `skip_serializing_if`.
//!
//! Many DTOs need to omit zero-valued fields to match the QQ Bot Open API's
//! `omitempty` semantics. Rather than redefining the same predicates in every
//! module, they live here and are imported via `crate::models::serde_helpers::*`.

/// True when `value` equals `T::default()`.
///
/// Use with `#[serde(skip_serializing_if = "is_default")]` on a bare typed
/// field that should be omitted at its zero value.
pub(crate) fn is_default<T>(value: &T) -> bool
where
    T: Default + PartialEq,
{
    *value == T::default()
}

/// True when `value` is `None`, or `Some(default)`.
///
/// Use with `#[serde(skip_serializing_if = "option_is_none_or_default")]` on
/// `Option<T>` fields where `None` and `Some(zero)` should both serialize as
/// "field absent". This matches Go's `omitempty` for pointer-to-zero values.
pub(crate) fn option_is_none_or_default<T>(value: &Option<T>) -> bool
where
    T: Default + PartialEq,
{
    value.as_ref().is_none_or(|v| *v == T::default())
}

/// True when an unsigned 32-bit value is zero.
pub(crate) fn is_zero_u32(value: &u32) -> bool {
    *value == 0
}

/// True when a signed 32-bit value is zero.
pub(crate) fn is_zero_i32(value: &i32) -> bool {
    *value == 0
}

/// True when a slice is empty.
///
/// Useful for bare `Vec<T>` fields that should be omitted when empty. Prefer
/// `Vec::is_empty` directly when the call site already imports it.
pub(crate) fn is_empty_vec<T>(value: &[T]) -> bool {
    value.is_empty()
}

/// Predicate for `bool` fields whose `false` value should be omitted.
///
/// Equivalent to `std::ops::Not::not` but easier to remember in this context.
pub(crate) fn is_false(value: &bool) -> bool {
    !*value
}

/// Deserialize an `Option<String>` from either a JSON string or number.
///
/// The QQ Bot Open API mixes string IDs and numeric IDs in some payloads
/// (timestamps, application IDs). This helper accepts either form so wire
/// structs can stay strongly typed.
pub(crate) fn deserialize_string_or_number<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        Str(String),
        I64(i64),
        U64(u64),
        F64(f64),
    }

    let value = Option::<StringOrNumber>::deserialize(deserializer)?;
    Ok(value.map(|v| match v {
        StringOrNumber::Str(s) => s,
        StringOrNumber::I64(n) => n.to_string(),
        StringOrNumber::U64(n) => n.to_string(),
        StringOrNumber::F64(n) => n.to_string(),
    }))
}
