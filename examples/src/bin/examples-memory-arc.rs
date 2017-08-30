use std::sync::{Arc, Weak};

struct Person(String);

impl Person {
    fn new(nom: &str) -> Person {
        Person(String::from(nom))
    }

    fn hello(&self) {
        println!("Hello {}", self.0);
    }
}

impl std::ops::Drop for Person {
    fn drop(&mut self) {
        println!("Goodbye {}", self.0);
    }
}

fn say_hello(person: Arc<Person>) {
    person.hello();
}

fn try_say_hello(person: Weak<Person>) {
    person.upgrade().map_or_else(
        | | println!("Person is gone ..."),
        |p| say_hello(p)
    );
}

fn main() {
    let robert = Arc::from(Person::new("ROBERT"));
    say_hello(robert);

    println!("\n--------------------------------\n");

    let durand = Arc::from(Person::new("DURAND"));
    say_hello(durand.clone());

    println!("\n--------------------------------\n");

    let weak_durand = Arc::downgrade(&durand);
    try_say_hello(weak_durand.clone());

    println!("\n--------------------------------\n");

    say_hello(durand);
    try_say_hello(weak_durand);

    println!("\n--------------------------------\n");
}
