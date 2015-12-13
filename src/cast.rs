use Castable;
use Constructable;
use UnsafeCastable;

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct Cast<T: Castable> {
    pub __box__: Box<UnsafeCastable>,
    p: PhantomData<T>
}

impl<T: Castable> Cast<T> {
    pub fn new(b: Box<UnsafeCastable>) -> Self {
        Cast { __box__: b, p: PhantomData }
    }

    pub fn cast_as<U: Castable>(self) -> Cast<U> {
        Cast::new(self.__box__)
    }
}

impl<T: Castable> Deref for Cast<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.__box__.downcast().unwrap()
    }
}

impl<T: Castable> DerefMut for Cast<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.__box__.downcast_mut().unwrap()
    }
}

impl<T: Constructable> From<T> for Cast<T> {
    fn from(t: T) -> Self {
        t.init()
    }
}
