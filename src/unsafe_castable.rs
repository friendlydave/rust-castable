use std::any::TypeId;
use std::mem;
use base::Base;

/// Low-level inspection and casting methods used by `Castable`.
///
/// The purpose of this trait is to expose important methods like u_downcast and u_cast
/// but without type parameters, which enables UnsafeCastable items to be accessed behind
/// a trait object. In general, you shouldn't need to call any of these methods yourself.
pub trait UnsafeCastable {
    /// Use a pointer from a `Box` to initialize `Base`.
    ///
    /// Without initializing `Base`, calls to `u_cast` will always fail with None.
    fn init_base(&mut self, s: Option<*mut UnsafeCastable>);

    /// Returns the `TypeId` of `Self`.
    ///
    /// It specifies `Self: Sized` so that `UnsafeCastable` can be converted to a trait object.
    fn ident() -> TypeId where Self: Sized;

    /// Returns the `TypeId` of `Self`
    ///
    /// This is an instance method so that the `TypeId` of trait objects can be accessed.
    /// Uses `ident` internally.
    fn get_ident(&self) -> TypeId;

    /// Returns the super struct as a trait object.
    fn get_super(&self) -> &UnsafeCastable;

    /// Returns the `Base` struct.
    ///
    /// Calls `get_base` on its super struct recursively until `Base` is reached, which
    /// ends the recursion by returning itself.
    fn get_base(&self) -> &Base {
        self.get_super().get_base()
    }

    /// Upcasts the type with a matching `TypeId`.
    ///
    /// Calls `u_upcast` on its super struct recursively. The recursion ends when either a
    /// super struct's `get_ident` matches `TypeId`, or when `Base` returns `None`. As the name
    /// implies, it only searches down from `self`.
    ///
    /// The type returned is a double reference to the actual type corresponding to the `TypeId`
    /// unsafely transmuted as `&&Base`, which gets around using a type parameter, but requires
    /// an unsafe transmute and careful planning.
    unsafe fn u_upcast(&self, t: TypeId) -> Option<&&Base> {
        if self.get_ident() == t {
            Some( mem::transmute::<&&Self, &&Base>(&self) )
        } else {
            self.get_super().u_upcast(t)
        }
    }

    /// Down/upcasts the type with a matching `TypeId`.
    ///
    /// Similar to `u_upcast`, but the search begins from the top of the type hierarchy regardless
    /// of what `self` is. This method uses `Base.instance`, and will fail with None if the
    /// `Base` was never initialized with `init_base`. This may be misleading, since type might
    /// have been found if `Base` were initialized.
    ///
    /// The type returned is a double reference to the actual type corresponding to the `TypeId`
    /// unsafely transmuted as `&&Base`, which gets around using a type parameter, but requires
    /// an unsafe transmute and careful planning.
    unsafe fn u_downcast(&self, t: TypeId) -> Option<&&Base> {
        if self.get_ident() == t {
            Some( mem::transmute::<&&Self, &&Base>(&self) )
        } else {
            self.get_base().instance.and_then(|inst| (&*inst).u_upcast(t) )
        }
    }
}
