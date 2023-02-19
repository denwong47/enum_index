//! Trait for getting a variant by name.

/// Get an Enum Variant by name in [`str`].
pub trait VariantByName
where
    Self: Sized,
{
    fn by_name(name: &str) -> Option<Self>;
}
