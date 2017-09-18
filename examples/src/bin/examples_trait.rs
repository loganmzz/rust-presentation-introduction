#[derive(Debug)]
struct Data(i32, i32);

impl std::ops::Add for Data {
    type Output = Data;

    fn add(self, rhs: Data) -> Data {
        Data(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn main() {
    println!("{:?}", Data(1,2) + Data(4,8));
}
