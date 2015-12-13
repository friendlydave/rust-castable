// in the event of macro failure, uncomment these lines:
//#![feature(trace_macros)]
//trace_macros!(true);

#[macro_use]
extern crate castable;

use castable::*;

inherit!{
    #[derive(Clone, Debug)]
    struct Person {
        name: String
    }

    #[derive(Clone, Debug)]
    struct Employee: Person {
        hours: f64,
        pay: f64
    }

    #[derive(Clone, Debug)]
    struct Salesperson: Employee {
        sales: u32
    }
}

impl Employee {
    fn income(&self) -> f64 {
        self.hours * self.pay
    }
}

#[test]
fn general() {
    // construct macro uses similar struct expression syntax
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
    // Salesperson Derefs to Person and Employee
    let msg = format!("{} made {} sales.", s.name, s.sales);
    assert_eq!(msg, "John made 4 sales.");
    assert_eq!(s.income(), 21.5 * 15.25);
    // implicit downcast to Person
    let p:&Person = &s;
    assert!(p.get_ident() == Person::ident());
    // explicit (up) cast to Employee
    let e:&Employee = p.downcast().unwrap();
    assert!(e.get_ident() == Employee::ident());
}

#[test]
fn mutability() {
    let mut s = construct!( Salesperson {
        sales: 4,
        sup.. Employee {
            hours: 21.5,
            pay: 15.25,
            sup.. Person {
                name: "John".to_string()
            }
        }
    });

    s.hours = 17.0;
    s.pay = 19.5;
    s.sales = 2;

    assert_eq!(format!("{} {} {} {}", s.name, s.hours, s.pay, s.sales), "John 17 19.5 2");

    let mut s2 = s.clone().init();
    {
        let p2:&mut Person = &mut s2;
        p2.name = "Dave".to_string();
        let e2:&mut Employee = p2.downcast_mut().unwrap();
        e2.hours += 3.0;
    }
    assert_eq!(format!("{} {}", s2.name, s2.hours), "Dave 20");
}
