use std::thread::{spawn, current};
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    
    spawn(move || {
        let mut scores = vec![1, 2];
        tx.send(scores);
    });
    
    spawn(move || tx2.send(vec![3, 4]));
    
    
    let scores: Vec<i32> = rx.recv().unwrap();
    println!("{:?}", scores); 
    
    // The code above display only one scores vector. 
    // You have to iterate on rx to get all the messages
    // Uncomment the code below and comment the line 17 and 18
    //
    //  for message in rx {
    //     println!("{}", message);
    // }
}