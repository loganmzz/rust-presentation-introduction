struct Personne {
    nom: String,
}

trait Etre {
    fn get_nom(&self) -> &str;
    fn se_presente(&self) {
        println!("Je suis {}", self.get_nom());
    }

    fn new(nom: &str) -> Self;
}

impl Etre for Personne {
    fn get_nom(&self) -> &str {
        self.nom.as_ref()
    }

    fn new(nom: &str) -> Self {
        Personne { nom: String::from(nom) }
    }
}

impl std::ops::Add for Personne {
    type Output = Personne;

    fn add(self, rhs: Personne) -> Personne {
        Personne { nom: format!("{}-{}", self.nom, rhs.nom) }
    }
}

trait Bipede: Etre {
    fn marcher_debout(&self) {
        self.se_presente();
        println!(" et je marche");
    }
}

impl Bipede for Personne {}

fn main() {
    let thomas = Personne { nom: String::from("THOMAS") };
    thomas.se_presente();
    
    let petit = Personne::new("PETIT");
    petit.se_presente();

    let thomas_petit = thomas + petit;
    thomas_petit.se_presente();

    thomas_petit.marcher_debout();
}
