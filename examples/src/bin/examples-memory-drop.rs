struct Person(String);

impl Person {
    fn new(name: &str) -> Person {
        Person(String::from(name))
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Goodbye {}", self.0)
    }
}

fn main() {
    println!("main: Begin");
    let richard = Person::new("RICHARD");
    drop(richard);
    println!("main: End");
}
