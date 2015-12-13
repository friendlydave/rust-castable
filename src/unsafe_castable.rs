use std::any::TypeId;
use std::mem;
use base::Base;

/// A trait-object safe implementation of downcasting using recursion.
///
/// Utilizes `std::any::TypeId` and unsafe `mem::transmute`s of borrows to avoid using type
/// parameters, enabling this trait to be coerced into a boxed trait-object. Without this
/// trait-object, dynamic downcasting would be impossible.
///
/// You should never have to implement or interact with this trait yourself. The `Castable`
/// trait provides a nice generic `downcast<T>() -> Option<&T>` method, and the `inherit!` and
/// `impl_inherit!` macros handle the implementing this trait on your types for you.
///
/// ```
/// # #![allow(dead_code)]
/// # #[macro_use]
/// # extern crate castable;
/// # use castable::UnsafeCastable;
/// # use castable::Constructable;
/// # inherit! {
/// #     #[derive(Default)] struct SuperType;
/// #     #[derive(Default)] struct SubType: SuperType;
/// # }
/// # fn main() {
/// let sub_type = &SubType::default().init();
/// let super_type = unsafe { sub_type.u_upcast(SuperType::ident()) };
/// assert!(super_type.is_some());
///
/// let super_type:&SuperType = sub_type;
/// let sub_type = unsafe { super_type.u_downcast(SubType::ident()) };
/// assert!(sub_type.is_some());
///
/// let super_type = &SuperType::default().init();
/// let sub_type = unsafe { super_type.u_downcast(SubType::ident()) };
/// assert!(sub_type.is_none());
/// # }
/// ```
pub trait UnsafeCastable {
    /// Use a pointer from a `Box` to initialize the `Base` super type.
    ///
    /// `*mut UnsafeCastable` points to the bottom most sub-type. It is implemented by
    /// accessing the super-type field and calling init_base recursively, until the super
    /// field is `Base`. `Base` then assigns this pointer to its `instance` field.
    fn init_base(&mut self, s: Option<*mut UnsafeCastable>);

    /// Returns the `TypeId` of `Self`.
    ///
    /// This is a type method, not an instance method. The `Sized` type constraint prevents
    /// this method from being implemented on the `UnsafeCastable` trait-object.
    fn ident() -> TypeId where Self: Sized;

    /// Returns the `TypeId` of `Self`
    ///
    /// During casting, this value is used to find the correct super-type.
    fn get_ident(&self) -> TypeId;

    /// Returns the super-type as a trait object.
    ///
    /// This is used to implment recursion during dynamic casting. Since the super-type is
    /// returned as a trait-object, `UnsafeCastable` remains type parameter free, and can
    /// itself be coerced into a trait-object.
    fn get_super(&self) -> &UnsafeCastable;

    /// Returns the `Base` type.
    ///
    /// Calls `get_base` on its super-type recursively until `Base` is reached, which
    /// ends the recursion by returning itself.
    fn get_base(&self) -> &Base {
        self.get_super().get_base()
    }

    /// Dynamically upcasts the type with a matching `TypeId`.
    ///
    /// Compares `get_ident()` with `TypeId`, return itself if it matches, otherwise
    /// calling `u_upcase` on its super-type. `Base` ends the recursion by returning `None`,
    /// meaning there is no type matching the `TypeId` within this instance's super-type
    /// hierarchy.
    ///
    /// The type returned is an unsafely transmuted double reference to the actual type
    /// corresponding to the `TypeId`, which gets around using a type parameter, but requires
    /// special handling by the caller.
    unsafe fn u_upcast(&self, t: TypeId) -> Option<&&UnsafeCastable> {
        if self.get_ident() == t {
            Some( mem::transmute::<&&Self, &&UnsafeCastable>(&self) )
        } else {
            self.get_super().u_upcast(t)
        }
    }

    /// Dynamically downcasts the type with a matching `TypeId`.
    ///
    /// Uses `get_base()` to access `Base.instance`, which is a pointer to the bottom most
    /// sub-type. Then, it calls `u_upcast()` and returns either the sub-type matching the
    /// `TypeId`, or `None` if not found. The type hierarchy acts as a single ended linked
    /// list, with `Base` holding a special reference back to the beginning. Downcasting
    /// is equivalent to upcasting starting from the bottom most sub-type.
    ///
    /// The type returned is an unsafely transmuted double reference to the actual type
    /// corresponding to the `TypeId`, which gets around using a type parameter, but requires
    /// special handling by the caller.
    unsafe fn u_downcast(&self, t: TypeId) -> Option<&&UnsafeCastable> {
        if self.get_ident() == t {
            Some( mem::transmute::<&&Self, &&UnsafeCastable>(&self) )
        } else {
            self.get_base().instance.and_then(|inst| (&*inst).u_upcast(t) )
        }
    }
}
