use std::any::TypeId;
use std::mem;

//type TypeIdent = u32;
pub type TypeIdent = TypeId;

pub trait Castable {
    fn ident() -> TypeIdent where Self: Sized;
    fn get_ident(&self) -> TypeIdent;
    fn get_super(&self) -> &Castable;

    fn get_base(&self) -> &Base {
        self.get_super().get_base()
    }

    fn cmp_ident(&self, t: TypeIdent) -> bool {
        self.get_ident() == t
    }

    fn inherits_from(&self, t: TypeIdent) -> bool {
        if self.cmp_ident(t) {
            true
        } else {
            self.get_super().inherits_from(t)
        }
    }

    fn u_downcast(&self, t: TypeIdent) -> Option<&&()> {
        if self.cmp_ident(t) {
            Some(unsafe { mem::transmute::<&&Self, &&()>(&self) })
        } else {
            self.get_super().u_downcast(t)
        }
    }

    fn u_upcast(&self, t: TypeIdent) -> Option<&&()> {
        if self.cmp_ident(t) {
            Some(unsafe { mem::transmute::<&&Self, &&()>(&self) })
        } else {
            unsafe { &*(self.get_base().true_inst) }.u_downcast(t)
        }
    }
}

pub trait Constructor: Castable {
    type Super: Castable;
    fn null() -> Self where Self: Sized {
        unsafe { mem::zeroed() }
    }
    fn init_base(&mut self, s: *const Castable);
    fn init() -> Self where Self: Sized + 'static {
        let mut s:Self = unsafe { mem::zeroed() };
        let p = unsafe { mem::transmute::<&Self, *const Self>(&s) };
        s.init_base(p);
        s
    }
    fn inherit(sup: Self::Super) -> Self;
}

pub struct Base {
    pub true_inst: *const Castable
}

impl Castable for Base {
    fn ident() -> TypeIdent {
        TypeId::of::<Base>()
    }

    fn get_ident(&self) -> TypeIdent {
        Self::ident()
    }

    fn get_super(&self) -> &Castable { self }
    fn get_base(&self) -> &Base { self }
    fn inherits_from(&self, _: TypeIdent) -> bool { false }
    fn u_downcast(&self, _: TypeIdent) -> Option<&&()> { None }
    fn u_upcast(&self, _: TypeIdent) -> Option<&&()> { None }
}

impl Constructor for Base {
    type Super = Base;
    fn init_base(&mut self, b: *const Castable) {
        self.true_inst = b;
    }
    fn inherit(_: Self::Super) -> Self {
        panic!("base cannot inherit from anything")
    }
}

pub trait CastableHelper: Castable {
    fn is<T: Castable>(&self) -> bool {
        self.cmp_ident(T::ident())
    }

    fn downcast<T: Castable>(&self) -> &T {
        let v = self.u_downcast(T::ident()).unwrap();
        *unsafe { mem::transmute::<&&(), &&T>(v) }
    }

    fn upcast<T: Castable>(&self) -> &T {
        let v = self.u_upcast(T::ident()).unwrap();
        *unsafe { mem::transmute::<&&(), &&T>(v) }
    }
}

impl CastableHelper for Base {}
//impl<T> CastableHelper for T where T: Castable {}
