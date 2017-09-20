use std::sync::Arc;
use std::thread::{spawn, current, JoinHandle};

fn main() {
    let reference = Arc::new(String::from("A shared string"));

    fn format_with_thread(reference: Arc<String>) -> JoinHandle<String> {
        spawn(move || format!("{:?} => {:?}", current().id(), reference))
    }

    let handles: Vec<_> = (0..2).map(|_| reference.clone())
                                .map(format_with_thread)
                                .collect();

    for handle in handles {
        println!("{}", handle.join().unwrap());
    }
}