#[derive(Clone, Debug)]
struct Data(i32, i32);

#[derive(Clone, Debug)]
struct Task {
    id: i64,
    data: Data,
}

impl Task {
    fn new(id: i64, data: Data) -> Task {
        Task { id, data }
    }

    fn data(&self) -> &Data {
        &self.data
    }
}

fn main() {
    let task = Task::new(1, Data(42, 314));
    println!("{:?}", task.data());
}
