use std::any::{Any, TypeId};
use std::marker::Reflect;
use std::mem;

struct Base {
    true_type: TypeId,
    true_inst: *const Castable
}

trait Castable: Reflect + 'static {
    // returns the derived-from struct
    fn get_super(&self) -> &Castable;

    fn get_base(&self) -> Base {
        self.get_super().get_base()
    }

    fn is<T: Castable>(&self) -> bool where Self: 'static {
        TypeId::of::<Self>() == TypeId::of::<T>()
    }

    fn inherits_from<T: Castable>(&self) -> bool {
        if self.is::<T>() {
            true
        } else {
            self.get_super().inherits_from::<T>()
        }
    }

    fn downcast<T: Castable>(&self) -> Option<&T> {
        if self.is::<T>() {
            Some(unsafe { mem::transmute::<Self, &T>(self) })
        } else {
            self.get_super().downcast::<T>()
        }
    }

    fn upcast<T: Castable>(&self) -> Option<&T> {
        if self.is::<T>() {
            Some(unsafe { mem::transmute::<Self, &T>(self) })
        } else {
            unsafe { &*(self.get_base().true_inst) }.downcast::<T>()
        }
    }
}
