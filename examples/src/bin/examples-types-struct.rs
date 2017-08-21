#![allow(dead_code,unused_variables)]

struct Vide;
struct SansChamps {}

struct Personne {
    nom: String,
    prenom: String,
}

struct Parents(Personne, Personne);
struct Famille {
    parents: Parents,
    enfants: Vec<Personne>,
}

impl Personne {
    fn new(nom: &str, prenom: &str) -> Personne {
        Personne { nom: String::from(nom), prenom: String::from(prenom) }
    }
}

fn main() {
    // Unit
    let vide = Vide;
    let sans_champs = SansChamps {};

    // Classique
    let dupont_jean = Personne { nom: String::from("MARTIN"), prenom: String::from("Jean") };

    // Tuple
    let parents = Parents(dupont_jean, Personne { nom: String::from("MARTIN"), prenom: String::from("Marie") });

    let enfants = vec![
        Personne { nom: String::from("MARTIN"), prenom: String::from("Philippe") },
        Personne { nom: String::from("MARTIN"), prenom: String::from("Michel")   }
    ];

    let duponts = Famille { parents, enfants };

    // impl
    let enfants = vec![ Personne::new("BERNARD", "Alain"), Personne::new("BERNARD", "Nathalie") ];
}
