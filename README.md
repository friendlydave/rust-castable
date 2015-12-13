# rust-cast
struct inheritance with automatic upcasting and dynamic downcasting.

I wanted something similar to how [the servo project](https://github.com/servo/servo/blob/master/components/script/dom/bindings/inheritance.rs) implemented inheritance for dom object, but with dynamic downcasting and without compiler plugins.

# Features

 - struct and impl inheritance
 - `inherit!` and `construct!` macros
 - straight-forward downcast method: `fn downcast<T>() -> Option<&T>`
 - upcasting uses `Deref` and `DerefMut` and is automatic
 - `Cast<T>` type enables heterogeneous containers

# Limitations

 - structs must be `Sized`
 - structs cannot use lifetime parameters (`struct Foo<'a>;`)
 - no macro support for structs with type parameters (`struct Foo<T>;`)
 - upcasting from `Cast<T>` uses `downcast()` internally, which means O(n)

# Example

(TODO: come up with a better example)

```rust

#[macro_use]
extern crate inheritance;

use inheritance::base::{Castable};

inherit!{
    pub struct Person {
        pub name: String
    }

    pub struct Employee: Person {
        pub hours: f64,
        pub pay: f64
    }

    pub struct Salesperson: Employee {
        pub sales: u32
    }
}

impl Employee {
    pub fn income(&self) -> f64 {
        self.hours * self.pay
    }
}

fn main() {
    let s = construct!( Salesperson {
        sales: 4,
        sup.. Employee {
            hours: 21.5,
            pay: 15.25,
            sup.. Person {
                name: "John".to_string()
            }
        }
    });
    let msg = format!("{} made {} sales.", s.name, s.sales);
    assert_eq!(msg, "John made 4 sales.");
    assert_eq!(s.income(), 21.5 * 15.25);
}
```
