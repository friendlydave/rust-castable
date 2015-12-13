use Cast;
use Castable;
use UnsafeCastable;

use std::mem;

/// Methods to ensure correct construction of Inheriable types.
///
/// When constructing `UnsafeCastable` types, consider the `construct!` macro before using these
/// methods; much like `UnsafeCastable` these methods are lowlevel and TODO
pub trait Constructable: Castable where Self: 'static {
    // the super-struct that Self inherits from
    type Super: Constructable;

    unsafe fn null() -> Self where Self: Sized {
        let mut s:Self = mem::zeroed();
        s.init_base(None);
        s
    }

    fn init<T: Castable>(self) -> Cast<T> where Self: Sized {
        let b:Box<UnsafeCastable> = Box::new(self);
        let bp = Box::into_raw(b);
        unsafe { &mut *bp }.init_base(Some(bp));
        Cast::new(unsafe { Box::from_raw(bp) })
    }

    unsafe fn inherit(sup: Self::Super) -> Self;

    fn clone_cast(&self) -> Cast<Self> where Self: Clone {
        Self::init(<Self as Clone>::clone(self))
    }
}
