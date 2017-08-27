use std::fmt;

enum Liste<T> {
    Nil,
    Cons(T, Box<Liste<T>>),
}

impl<T: fmt::Display> fmt::Display for Liste<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut liste: &Liste<_> = self;
        while let &Liste::Cons(ref val, ref tail) = liste {
            write!(f, "{}", val)?;
            liste = &*tail;
            if let &Liste::Cons(_, _) = liste {
                write!(f, ", " )?;
            }
        }
        write!(f, "]")
    }
}

trait Foobar: fmt::Debug {}

fn consume(foobar: Box<Foobar>) {
    println!("{:?}", foobar);
}

fn main() {
    let liste = Liste::Cons(1, Box::new(Liste::Cons(2, Box::new(Liste::Nil))));
    println!("{}", liste);
}
