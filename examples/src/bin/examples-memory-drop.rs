struct Personne(String);

impl Personne {
    fn new(nom: &str) -> Personne {
        Personne(String::from(nom))
    }
}

impl Drop for Personne {
    fn drop(&mut self) {
        println!("Au revoir {}", self.0)
    }
}

fn main() {
    println!("main: Début");
    let richard = Personne::new("RICHARD");
    drop(richard);
    println!("main: Fin");
}
