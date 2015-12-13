use super::employee_setup::*;
use super::super::Cast;
use super::super::Castable;

fn inspect_employee(e: &Employee) {
    println!("{} has worked {} hours this week", e.name, e.hours);
}

#[test]
fn generic_casting() {
    // using the generic keyword, construct! returns a Box<Inheritable>
    let ref g:Cast<Employee> = construct!( Salesperson as Employee {
        sales: 4,
        sup.. Employee {
            hours: 12.5,
            pay: 15.25,
            sup.. Person {
                name: "John 'Lazy-Worker' Doe".to_string()
            }
        }
    });
    // we can cast as Salesperson
    assert!(g.downcast::<Salesperson>().is_some());

    inspect_employee(g);
    // put a regular employee in the same variable
    let ref g:Cast<Employee> = construct!( Employee {
        hours: 42.0,
        pay: 16.15,
        sup.. Person {
            name: "Billy 'Hard-Worker' Smith".to_string()
        }
    });
    // we can no longer cast as Salesperson
    assert!(g.downcast::<Salesperson>().is_none());
    inspect_employee(g);
}
