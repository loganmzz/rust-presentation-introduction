use std::thread::spawn;
use std::sync::mpsc;

fn main() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    
    spawn(move || {
        let mut scores = vec![1, 2];
        tx1.send(scores)
        // ; scores.push(125);
    });
    
    spawn(move || tx2.send(vec![3, 4]));
    
    
    for scores in rx {
        println!("{:?}", scores);
    }
}