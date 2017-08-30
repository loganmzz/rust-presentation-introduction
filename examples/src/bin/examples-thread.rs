fn display_1_to_10() -> std::thread::ThreadId {
    let thread = std::thread::current();
    let thread_name = thread.name().unwrap_or("<unknown>");
    for i in 1..11 {
        println!("{}: {}", thread_name, i);
    }
    thread.id()
}

fn main() {
    let handles: Vec<_> = (0..2).map(|n| format!("Thread {}", n))
                        .map(|thread_name|
                          std::thread::Builder::new()
                                               .name(thread_name.clone())
                                               .spawn(display_1_to_10)
                                               .expect(&format!("Error launching {}", thread_name))
                        )
                        .collect();
    for handle in handles {
        let thread = String::from(handle.thread().name().unwrap());
        let resultat = handle.join()
                             .expect("Error during result generation");

        println!("Result: {}={:?}", thread, resultat);
    }
}
