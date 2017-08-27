fn afficher_1_a_10() -> std::thread::ThreadId {
    let thread = std::thread::current();
    let thread_name = thread.name().unwrap_or("<inconnu>");
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
                                               .spawn(afficher_1_a_10)
                                               .expect(&format!("Erreur lors du lancement du {}", thread_name))
                        )
                        .collect();
    for handle in handles {
        let thread = String::from(handle.thread().name().unwrap());
        let resultat = handle.join()
                             .expect("Erreur lors de la génération du resultat");

        println!("Résultat: {}={:?}", thread, resultat);
    }
}
