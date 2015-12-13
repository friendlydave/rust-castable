use UnsafeCastable;
use Base;
use std::mem;

// high-level inspecting and casting methods powered by UnsafeCastable
pub trait Castable: UnsafeCastable {
    // attempts to downcast Self to T, looking at types any type associated with Self
    fn downcast<T: UnsafeCastable>(&self) -> Option<&T> {
        if let Some(v) = unsafe { self.u_downcast(T::ident()) } {
            Some(*unsafe { mem::transmute::<&&Base, &&T>(v) })
        } else {
            None
        }
    }
    
    // attempts to down-cast Self to T, cheaper than Castable::cast
    fn downcast_mut<T: UnsafeCastable>(&mut self) -> Option<&mut T> {
        unsafe { mem::transmute::<Option<&T>, Option<&mut T>>(self.downcast()) }
    }
}

impl Castable for UnsafeCastable {}
