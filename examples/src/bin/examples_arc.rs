use std::sync::Arc;

mod examples_box;
use examples_box::Person;

fn main() {
    let stack = Person::new("stack");
    println!("stack={:?} ({:p})", stack.name(), &stack);

    let refc0 = Arc::new(Person::new("refctr"));
    println!("refc0={:?} ({:p})", refc0.name(), refc0);
    let refc1 = refc0.clone();
    println!("refc1={:?} ({:p})", refc1.name(), refc1);

    fn consume(refcn: Arc<Person>) {
        println!("refcn={:?} ({:p})", refcn.name(), refcn);
        println!("-- End of consume --");
    }
    consume(refc0.clone());
    consume(refc0.clone());
    println!("-- End of main --");
}