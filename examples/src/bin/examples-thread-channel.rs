use std::sync::mpsc::channel;
use std::thread::{spawn, current};

fn main() {

    let (tx, rx) = channel();
    
    for i in 0..3 {
        let tx = tx.clone();
        spawn(move || tx.send(format!("{:?} => {}", current().id(), i)).unwrap());
    }

    drop(tx);
    for message in rx {
        println!("{}", message);
    }
}