use UnsafeCastable;

/// A safe implementation for downcasting using type parameters.
///
/// Types inheriting from `Base` can be casted up and down. Upcasting is automatic through
/// the use of `Deref`; downcasting is explicit through the use of the `downcast` method defined
/// on this trait. Types created using the `inherit!` or `impl_inherit!` macros will implement
/// this trait for you.
///
/// Downcasting is dynamcic, and will fail by returning None if the downcast cannot be completed.
/// The return type is `Option<T>`, so it is advised to use a match expression instead of
/// `unwrap()` unless you are confident the downcast won't fail.
///
/// ```
/// # #![allow(dead_code)]
/// # #[macro_use]
/// # extern crate castable;
/// # use castable::Castable;
/// # use castable::Constructable;
/// # inherit! {
/// #     #[derive(Default, Clone)] struct SuperType;
/// #     #[derive(Default, Clone)] struct SubType: SuperType;
/// # }
/// # fn main() {
/// let sub_type = &SubType::default().init();
/// // an implicit upcast/deref
/// let super_type:&SuperType = sub_type;
/// // an explicit upcast using the upcast! macro
/// let super_type = upcast!(sub_type as SuperType);
/// // an explicit downcast
/// assert!(super_type.downcast::<SubType>().is_some());
/// # }
/// ```
///
/// Downcasting will only work if a type is correctly constructed, i.e. wrapped in a `Cast<T>`
/// object. Consult the documentation for the `Constructable` trait and `Cast` type for details
/// on correctly constructed types.
pub trait Castable: UnsafeCastable {
    /// Dynamically downcasts Self to T, returning None on failure.
    fn downcast<T: UnsafeCastable>(&self) -> Option<&T> {
        if let Some(any) = unsafe { self.u_downcast(T::ident()) } {
            any.downcast_ref::<T>()
        } else {
            None
        }
    }

    /// Dynamically downcasts Self to T, returning None on failure.
    fn downcast_mut<T: UnsafeCastable>(&mut self) -> Option<&mut T> {
        if let Some(any) = unsafe { self.u_downcast_mut(T::ident()) } {
            any.downcast_mut::<T>()
        } else {
            None
        }
    }
}

/// Implement `Castable` for the `UnsafeCastable` trait-object.
impl Castable for UnsafeCastable {}
