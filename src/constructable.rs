use Cast;
use Castable;
use UnsafeCastable;

use std::mem;

/// Methods to ensure correct construction of Inheriable types.
///
/// When constructing `UnsafeCastable` types, consider the `construct!` macro before using these
/// methods; much like `UnsafeCastable` these methods are lowlevel and TODO
pub trait Constructable: Castable {
    // the super-struct that Self inherits from
    type Super: Constructable;

    unsafe fn null() -> Self where Self: Sized {
        let mut s:Self = mem::zeroed();
        s.init_base(None);
        s
    }

    fn default() -> Self where Self: Sized {
        unimplemented!();
    }

    fn init<T: Castable>(s: Self) -> Cast<T> where Self: Sized + 'static {
        let b:Box<UnsafeCastable> = Box::new(s);
        let bp = Box::into_raw(b);
        unsafe { &mut *bp }.init_base(Some(bp));
        Cast::new(unsafe { Box::from_raw(bp) })
    }

    unsafe fn inherit(sup: Self::Super) -> Self;

    fn clone_cast(&self) -> Cast<Self> where Self: Clone + 'static {
        Self::init(self.clone())
    }
}
