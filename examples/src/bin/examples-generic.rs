use std::fmt;

trait Container<T> {
    fn get(&self) -> T;
    fn add(&self, that: &Self) -> Self;
}

#[derive(Debug)]
struct Singleton<T>(T);

impl Container<i64> for Singleton<i64> {
    fn get(&self) -> i64 {
        let &Singleton(value) = self;
        value
    }

    fn add(&self, that: &Singleton<i64>) -> Singleton<i64> {
        Singleton(self.get() + that.get())
    }
}


trait Adaptable<T> {
    fn adapt(&self) -> T;
}

impl Adaptable<Vec<i64>> for Singleton<i64> {
    fn adapt(&self) -> Vec<i64> {
        vec![self.get()]
    }
}

impl Adaptable<String> for Singleton<i64> {
    fn adapt(&self) -> String {
        format!("{:?}", self)
    }
}

fn display(a: String) {
    println!("{:?}", a);
}


impl<T: fmt::Display> fmt::Display for Singleton<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.0)
    }
}


trait AffichageSingleton {
    fn affiche_singleton(self);
}

impl<T> AffichageSingleton for T where Singleton<T>: fmt::Display {
    fn affiche_singleton(self) {
        println!("affichage singleton => {}", Singleton(self));
    }
}


fn main() {
    let deux = Singleton(2);
    let six = Singleton(6);
    println!("debug 2+6 => {:?}", deux.add(&six));


    let deux_into_vec: Vec<_> = deux.adapt();
    let deux_into_str = deux.adapt();
    println!("debug vec => {:?}  /  debug str => {:?}", deux_into_vec, deux_into_str);
    display(deux_into_str);


    println!("display deux => {}", deux);
    let liste = Singleton(vec!(1, 2, 3));
    println!("debug liste => {:?}", liste);
    // println!("{}", liste);

    42.affiche_singleton();
    // deux_into_vec.affiche_singleton();
}
