inherit!{
    #[derive(Clone, Debug)]
    pub struct Person {
        pub name: String
    }

    #[derive(Clone, Debug)]
    pub struct Employee: Person {
        pub hours: f64,
        pub pay: f64
    }

    #[derive(Clone, Debug)]
    pub struct Salesperson: Employee {
        pub sales: u32
    }
}

impl Employee {
    pub fn income(&self) -> f64 {
        self.hours * self.pay
    }
}
