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


trait SingletonDisplay {
    fn display_singleton(self);
}

impl<T> SingletonDisplay for T where Singleton<T>: fmt::Display {
    fn display_singleton(self) {
        println!("display singleton => {}", Singleton(self));
    }
}


fn main() {
    let two = Singleton(2);
    let six = Singleton(6);
    println!("debug 2+6 => {:?}", two.add(&six));


    let two_into_vec: Vec<_> = two.adapt();
    let two_into_str = two.adapt();
    println!("debug vec => {:?}  /  debug str => {:?}", two_into_vec, two_into_str);
    display(two_into_str);


    println!("display two => {}", two);
    let list = Singleton(vec!(1, 2, 3));
    println!("debug list => {:?}", list);
    // println!("{}", list);

    42.display_singleton();
    // two_into_vec.display_singleton();
}
