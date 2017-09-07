enum Operations {
    ADD,
    MUL,
    DIV
}

#[derive(Clone, Debug)]
struct Task {
    id: i64,
    data: i32,
}

fn main() {
    let tasks_queue = retrieve_tasks();

    for (task, operation) in tasks_queue {
       let res = compute_operation(task.data, operation); 
       println!("Task {} give {}", task.id, res);
    }
}

fn compute_operation(data: i32, operation: Operations) -> String {
    let res: String;

    if let  Operations::ADD = operation {
        res = add_operation(data);
    }
    else if let Operations::MUL = operation {
        res = mul_operation(data);
    }
    else {
        res = div_operation(data);
    }

    return res;
}

fn retrieve_tasks() -> Vec< (Task, Operations) > {
    vec![
        ( Task{ id: 1, data: 1}, Operations::ADD ),
        ( Task{ id: 2, data: 2}, Operations::MUL ),
        ( Task{ id: 3, data: 2}, Operations::MUL ),
        ( Task{ id: 4, data: 30}, Operations::DIV ),
    ]
}

fn add_operation(data: i32) -> String {
    let compute = data + 10;
    return String::from(compute.to_string())
}

fn mul_operation(data: i32) -> String {
    let compute = data * 10;
    return String::from(compute.to_string())
}

fn div_operation(data: i32) -> String {
    let compute = data / 10;
    return String::from(compute.to_string())
}