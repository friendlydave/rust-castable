use Cast;
use Castable;
use UnsafeCastable;

/// Methods to ensure the correct initialization of types inheriting from `Base`.
///
/// Types can be constructed without being wrapped in `Cast<T>` objects by using struct
/// expressions manually and with the `construct!( raw ... )` macro. However, types constructed
/// this way are downcast succeeded; the `downcast::<T>()` method will always return None.
///
/// ```
/// # #![allow(dead_code)]
/// # #[macro_use]
/// # extern crate castable;
/// # use castable::Castable;
/// # use castable::Constructable;
/// # inherit! {
/// #     #[derive(Default)] struct SuperType;
/// #     #[derive(Default)] struct SubType: SuperType;
/// # }
/// # fn main() {
/// // a correctly constructed type (call init(), converts to Cast<SubType>)
/// let sub_type = &SubType::default().init();
/// let super_type:&SuperType = sub_type;
/// // downcast succeeded
/// assert!(super_type.downcast::<SubType>().is_some());
///
/// // an incorrectly constructed type (not init(), unconverted SubType)
/// let sub_type = &SubType::default();
/// let super_type:&SuperType = sub_type;
/// // downcast failed
/// assert!(super_type.downcast::<SubType>().is_none());
/// # }
/// ```
///
/// If a type inherits from `Base` and also implemented the `Clone` trait, the `clone()` method
/// ends up creating an incorrectly constructed type, which will the same limitations mentioned
/// above. If you want to create a correctly constructed clone of a type, call `clone().init()`.
/// It should be noted that calling `clone().init()` will not create a clone of the bottom level
/// subtype, but a clone of the currently casted type.
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
/// // Cast<SubType> upcasted/dereffed to &SuperType
/// let super_type:&SuperType = sub_type;
/// // &SuperType.clone() returns a raw SuperType, which doesn't know about SubType
/// let super_type = &super_type.clone().init();
/// // downcast failed
/// assert!(super_type.downcast::<SubType>().is_none());
///
/// // expected behavior is achieved by downcasting before calling clone
/// let super_type:&SuperType = &sub_type.downcast::<SubType>().unwrap().clone().init();
/// // downcast succeeded
/// assert!(super_type.downcast::<SubType>().is_some());
/// # }
/// ```
pub trait Constructable: Castable where Self: 'static {
    /// The super-type that `Self` inherits from.
    ///
    /// Used by the `construct!` macro to call the super-type's `Default::default()` when the
    /// value of the super-type is omitted.
    type Super: Constructable;

    /// Wraps an incorrectly constructed type in a `Cast<Self>` object.
    ///
    /// As mentioned above, this method converts an incorrectly constructed type into a correctly
    /// constructed type, allowing downcasting.
    fn init(self) -> Cast<Self> where Self: Sized {
        let b:Box<UnsafeCastable> = Box::new(self);
        let bp = Box::into_raw(b);
        unsafe { &mut *bp }.init_base(Some(bp));
        Cast::new(unsafe { Box::from_raw(bp) })
    }
}
