use Castable;
use Constructable;
use UnsafeCastable;

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct Cast<T: Castable> {
    pub b: Box<UnsafeCastable>,
    p: PhantomData<T>
}

impl<T: Castable> Cast<T> {
    pub fn new(b: Box<UnsafeCastable>) -> Self {
        Cast { b: b, p: PhantomData }
    }

    pub fn cast_as<U: Castable>(self) -> Cast<U> {
        Cast::new(self.b)
    }
}

impl<T: Castable> Deref for Cast<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.b.downcast().unwrap()
    }
}

impl<T: Castable> DerefMut for Cast<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.b.downcast_mut().unwrap()
    }
}

impl<T: Clone + Constructable + 'static> Clone for Cast<T> {
    fn clone(&self) -> Self {
        T::clone_cast(self)
    }
}
