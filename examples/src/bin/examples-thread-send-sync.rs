use std::fmt::{Debug, Formatter, Result};
use std::thread;
use std::sync::Arc;

struct Person(String);

impl Debug for Person {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Hello {}", self.0)
    }
}

fn into_thread<T: Debug + Send + 'static>(data: T) {
    thread::spawn(move || {
        println!("{:?} => {:?}", thread::current().id(), data);
    }).join().unwrap();
}

fn into_threads<T: Debug + Send + Sync + 'static>(data: T) {
    let reference = Arc::new(data);
    let handles: Vec<_> = (0..4)
        .map(|_| reference.clone())
        .map(|shared| std::thread::spawn(move || println!("{:?} => {:?}", std::thread::current().id(), shared)))
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    into_thread(Person(String::from("DUBOIS")));
    // into_thread(std::rc::Rc::new(Person(String::from("MOREAU"))));

    into_thread("Hello LAURENT");

    into_threads(Person(String::from("SIMON")));
    // into_thread(std::rc::Rc::new(Person(String::from("MICHEL"))));
}