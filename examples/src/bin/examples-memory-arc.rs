use std::sync::{Arc, Weak};

struct Personne(String);

impl Personne {
    fn new(nom: &str) -> Personne {
        Personne(String::from(nom))
    }

    fn bonjour(&self) {
        println!("Bonjour {}", self.0);
    }
}

impl std::ops::Drop for Personne {
    fn drop(&mut self) {
        println!("Au revoir {}", self.0);
    }
}

fn dis_bonjour(personne: Arc<Personne>) {
    personne.bonjour();
}

fn essaie_de_dire_bonjour(personne: Weak<Personne>) {
    personne.upgrade().map_or_else(
        | | println!("La personne est partie ..."),
        |p| dis_bonjour(p)
    );
}

fn main() {
    let robert = Arc::from(Personne::new("ROBERT"));
    dis_bonjour(robert);

    println!("\n--------------------------------\n");

    let durand = Arc::from(Personne::new("DURAND"));
    dis_bonjour(durand.clone());

    println!("\n--------------------------------\n");

    let durand_faible = Arc::downgrade(&durand);
    essaie_de_dire_bonjour(durand_faible.clone());

    println!("\n--------------------------------\n");

    dis_bonjour(durand);
    essaie_de_dire_bonjour(durand_faible);

    println!("\n--------------------------------\n");
}
