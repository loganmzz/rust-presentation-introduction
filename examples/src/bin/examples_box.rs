#[derive(Debug)]
pub struct Person {
    name: String,
}

impl Person {
    pub fn new(name: &str) -> Person {
        Person { name: String::from(name) }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Drop of {:?}", self);
    }
}


fn main() {
    let stack = Person::new("stack");
    println!("stack={:?}", stack.name());

    let boxed = Box::new(Person::new("boxed"));
    println!("boxed={:?}", boxed.name());

    fn consume(boxed: Box<Person>) {
        println!("boxed={:?}", boxed.name());
        println!("-- End of consume --");
    }
    consume(boxed);
    // consume(boxed);
    println!("-- End of main --");
}
