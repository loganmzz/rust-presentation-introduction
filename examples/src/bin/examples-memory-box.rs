use std::fmt;

enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    fn from_vec(data: Vec<T>) -> List<T> {
        data.into_iter().rev().fold(List::Nil, |list, e| List::new(e, list))
    }
    fn new(value: T, child: List<T>) -> List<T> {
        List::Cons(value, Box::new(child))
    }

    fn get(&self, index: usize) -> Option<&T> {
        if let &List::Cons(ref value, ref child) = self {
            if index == 0 {
                Some(value)
            } else {
                child.get(index - 1)
            }
        } else {
            None
        }
    }
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
    let list = List::from_vec(vec![1, 2]);
    println!("{}", list);

    println!("[1]={:?}", list.get(1));
    println!("[2]={:?}", list.get(2));
}
