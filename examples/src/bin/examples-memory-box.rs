use std::fmt;

enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut list: &List<_> = self;
        while let &List::Cons(ref val, ref tail) = list {
            write!(f, "{}", val)?;
            list = &*tail;
            if let &List::Cons(_, _) = list {
                write!(f, ", " )?;
            }
        }
        write!(f, "]")
    }
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{}", list);
}
