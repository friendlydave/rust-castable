use std::any::TypeId;
use std::mem;
use std::fmt;

// used for checking casts
pub type TypeIdent = TypeId;

// low-level inspecting and casting methods used by Castable
pub trait Inheritable {
    // trait method to get identity of type
    fn ident() -> TypeIdent where Self: Sized;
    // method to get identity of type (uses Inheritable::ident())
    fn get_ident(&self) -> TypeIdent;
    // gets the super-struct as a castable trait object
    fn get_super(&self) -> &Inheritable;
    // gets the base struct recursively
    fn get_base(&self) -> &Base {
        self.get_super().get_base()
    }
    // searches for the super-struct of matching type identity
    fn u_downcast(&self, t: TypeIdent) -> Option<&&Base> {
        if self.get_ident() == t {
            Some(unsafe { mem::transmute::<&&Self, &&Base>(&self) })
        } else {
            self.get_super().u_downcast(t)
        }
    }
    // searches for the super-struct from the top
    fn u_cast(&self, t: TypeIdent) -> Option<&&Base> {
        if self.get_ident() == t {
            Some(unsafe { mem::transmute::<&&Self, &&Base>(&self) })
        } else {
            unsafe { &*self.get_base().true_inst.expect(
                "unable to cast non Box<_> type") }.u_downcast(t)
        }
    }
}

// correct construction of inherited structs
pub trait Constructable: Inheritable {
    // the super-struct that Self inherits from
    type Super: Constructable;
    fn null() -> Self where Self: Sized {
        let mut s:Self = unsafe { mem::zeroed() };
        s.init_base(None);
        s
    }
    fn init_base(&mut self, s: Option<*mut Inheritable>);
    fn init(s: Self) -> Box<Self> where Self: Sized + 'static {
        let b = Box::new(s);
        let bp = Box::into_raw(b);
        unsafe { &mut *bp }.init_base(Some(bp));
        unsafe { Box::from_raw(bp) }
    }
    fn inherit(sup: Self::Super) -> Self;

    fn boxed_clone(&self) -> Box<Self> where Self: Clone + 'static {
        Self::init(Clone::clone(self))
    }
}


// high-level inspecting and casting methods powered by Inheritable
pub trait Castable: Constructable {
    // returns true if Self can Castable::cast to T
    fn is<T: Constructable>(&self) -> bool {
        self.u_cast(T::ident()).is_some()
    }
    // attempts to down-cast Self to T, cheaper than Castable::cast
    fn downcast<T: Constructable>(&self) -> Option<&T> {
        if let Some(v) = self.u_downcast(T::ident()) {
            Some(*unsafe { mem::transmute::<&&Base, &&T>(v) })
        } else {
            None
        }
    }
    // attempts to down-cast Self to T, cheaper than Castable::cast
    fn downcast_mut<T: Constructable>(&mut self) -> Option<&mut T> {
        unsafe { mem::transmute::<Option<&T>, Option<&mut T>>(self.downcast()) }
    }
    // attempts to cast Self to T, either up or down
    fn cast<T: Constructable>(&self) -> Option<&T> {
        if let Some(v) = self.u_cast(T::ident()) {
            Some(*unsafe { mem::transmute::<&&Base, &&T>(v) })
        } else {
            None
        }
    }
    // attempts to cast Self to T, either up or down
    fn cast_mut<T: Constructable>(&mut self) -> Option<&mut T> {
        unsafe { mem::transmute::<Option<&T>, Option<&mut T>>(self.cast()) }
    }
}

pub struct Base {
    pub true_inst: Option<*mut Inheritable>
}

impl Clone for Base {
    fn clone(&self) -> Self {
        Base { true_inst: None }
    }
}

impl fmt::Debug for Base {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "...")
    }
}

impl Inheritable for Base {
    fn ident() -> TypeIdent {
        TypeId::of::<Base>()
    }

    fn get_ident(&self) -> TypeIdent {
        Self::ident()
    }

    fn get_super(&self) -> &Inheritable { self }
    fn get_base(&self) -> &Base { self }
    fn u_downcast(&self, _: TypeIdent) -> Option<&&Base> { None }
    fn u_cast(&self, _: TypeIdent) -> Option<&&Base> { None }
}

impl Constructable for Base {
    type Super = Base;
    fn init_base(&mut self, b: Option<*mut Inheritable>) {
        self.true_inst = b;
    }
    fn inherit(_: Self::Super) -> Self {
        panic!("base cannot inherit from anything")
    }
}

impl Castable for Base {}
