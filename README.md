# rust-inherit
basic struct inheritance in rust

I originally looked at how [the servo project](https://github.com/servo/servo/blob/master/components/script/dom/bindings/inheritance.rs) implemented inheritance for the dom, but this doesn't use complier plugins or unsafe casting (only to get around life-times).

# Example

Below is an example demonstrating basic usage of this crate.

```rust

#[macro_use]
extern crate inheritance;

use inheritance::base::{Constructor, CastableHelper};

inherit!{
    pub struct Person {
        name: String
    }

    pub struct Worker: Person {
        job: String
    }
}

fn main() {
    // TODO: improve constructors
    let w = Worker {
        job: "sales".to_string(),
        .. Worker::inherit(Person {
            name: "John".to_string(),
            .. Person::null()
        })
    };
    let msg = format!("{} works in {}.", w.downcast::<Person>().name, w.job);
    assert_eq!(msg, "John works in sales.");
}
```

# Planned Features
I hope to improve the constructors, to implement down/upcast_mut, and Deref for auto-down/upcasting.
