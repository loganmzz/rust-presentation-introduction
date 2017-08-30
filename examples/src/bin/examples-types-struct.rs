#![allow(dead_code,unused_variables)]

struct Void;
struct NoAttributes {}

struct Person {
    last_name: String,
    first_name: String,
}

struct Parents(Person, Person);
struct Family {
    parents: Parents,
    children: Vec<Person>,
}

impl Person {
    fn new(last_name: &str, first_name: &str) -> Person {
        Person { last_name: String::from(last_name), first_name: String::from(first_name) }
    }
}

fn main() {
    // Unit
    let void = Void;
    let no_fields = NoAttributes {};

    // Classique
    let dupont_jean = Person { last_name: String::from("MARTIN"), first_name: String::from("Jean") };

    // Tuple
    let parents = Parents(dupont_jean, Person { last_name: String::from("MARTIN"), first_name: String::from("Marie") });

    let children = vec![
        Person { last_name: String::from("MARTIN"), first_name: String::from("Philippe") },
        Person { last_name: String::from("MARTIN"), first_name: String::from("Michel")   }
    ];

    let duponts = Family { parents, children };

    // impl
    let children = vec![ Person::new("BERNARD", "Alain"), Person::new("BERNARD", "Nathalie") ];
}
