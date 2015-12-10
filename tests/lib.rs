// in the event of macro failure, comment these lines:
//#![feature(trace_macros)]
//trace_macros!(true);

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

#[test]
fn general() {
    let w = Worker {
        job: "sales".to_string(),
        .. Worker::inherit(Person {
            name: "John".to_string(),
            .. Person::null()
        })
    };
    let msg = format!("{} works in {}.",w.downcast::<Person>().name,w.job);
    assert_eq!(msg, "John works in sales.");
}
