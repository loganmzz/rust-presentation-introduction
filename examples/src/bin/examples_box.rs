mod examples_struct;
use examples_struct::{Data, Task};

fn main() {
    let stack = Task::new(0, Data(0, 0));
    println!("stack={:?}", stack.data());

    let boxed = Box::new(Task::new(1, Data(0, 1)));
    println!("boxed={:?}", boxed.data());

    fn consume(boxed: Box<Task>) {
        println!("boxed={:?}", boxed.data());
    }
    consume(boxed);
    // consume(boxed);
}