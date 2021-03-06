use super::employee_setup::*;
use super::super::*;

#[test]
fn general_casting() {
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
fn mutable_casting() {
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

    assert_eq!(format!("{} {} {} {}", s.name, s.hours, s.pay, s.sales),
    "John 17 19.5 2");

    let mut s2 = s.clone().init();
    {
        let p2:&mut Person = &mut s2;
        p2.name = "Dave".to_string();
        let e2:&mut Employee = p2.downcast_mut().unwrap();
        e2.hours += 3.0;
    }
    assert_eq!(format!("{} {}", s2.name, s2.hours), "Dave 20");
}
