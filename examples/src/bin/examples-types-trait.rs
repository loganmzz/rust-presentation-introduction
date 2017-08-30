struct Person {
    name: String,
}

trait Being {
    fn get_name(&self) -> &str;
    fn introduce(&self) {
        println!("I am {}", self.get_name());
    }

    fn new(name: &str) -> Self;
}

impl Being for Person {
    fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    fn new(name: &str) -> Self {
        Person { name: String::from(name) }
    }
}

impl std::ops::Add for Person {
    type Output = Person;

    fn add(self, rhs: Person) -> Person {
        Person { name: format!("{}-{}", self.name, rhs.name) }
    }
}

trait Biped: Being {
    fn standed_walk(&self) {
        self.introduce();
        println!(" and I walk");
    }
}

impl Biped for Person {}

fn main() {
    let thomas = Person { name: String::from("THOMAS") };
    thomas.introduce();
    
    let petit = Person::new("PETIT");
    petit.introduce();

    let thomas_petit = thomas + petit;
    thomas_petit.introduce();

    thomas_petit.standed_walk();
}
