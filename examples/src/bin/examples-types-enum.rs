#![allow(dead_code,unused_variables)]

enum Personne {
    Anonyme,
    Physique { nom: String, prenom: String },
    Entreprise { raison_sociale: String, proprietaires: Vec<Personne> },
    Association(String),
}

impl Personne {
    fn afficher_message(self) {
        match self {
            Personne::Anonyme => println!("Je ne suis personne"),
            Personne::Physique { nom, prenom } => println!("Je me présente, je m'appelle {} {}", prenom, nom),
            Personne::Entreprise { raison_sociale, .. } => println!("$$$ {} $$$", raison_sociale),
            Personne::Association(nom) => println!("❤❤❤ {} ❤❤❤", nom)
        }
    }
}

fn main() {
    let personne: Personne = Personne::Association(String::from("WWF"));
    personne.afficher_message();
}
