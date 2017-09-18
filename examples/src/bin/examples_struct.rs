#[derive(Clone, Debug)]
pub struct Data(pub i32, pub i32);

#[derive(Clone, Debug)]
pub struct Task {
    id: i64,
    data: Data,
}

impl Task {
    pub fn new(id: i64, data: Data) -> Task {
        Task { id, data }
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}

fn main() {
    let task = Task::new(1, Data(42, 314));
    println!("{:?}", task.data());
}
