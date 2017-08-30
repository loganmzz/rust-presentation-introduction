#![allow(dead_code,unused_variables)]

enum Person {
    Anonymous,
    Natural { last_name: String, first_name: String },
    Company { name: String, owners: Vec<Person> },
    Association(String),
}

impl Person {
    fn afficher_message(self) {
        match self {
            Person::Anonymous => println!("I am no one"),
            Person::Natural { last_name, first_name } => println!("Hello, my name is {} {}", first_name, last_name),
            Person::Company { name, .. } => println!("$$$ {} $$$", name),
            Person::Association(nom) => println!("❤❤❤ {} ❤❤❤", nom)
        }
    }
}

fn main() {
    let personne: Person = Person::Association(String::from("WWF"));
    personne.afficher_message();
}
