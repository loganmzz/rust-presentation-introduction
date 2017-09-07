#![feature(iterator_for_each)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

enum Operations {
    ADD,
    MUL,
    DIV
}

enum OperationsError {
    UNKNOW_OPERATION,
    OVERFLOW,
    PARSING,
}

#[derive(Clone, Debug)]
struct Task {
    id: i64,
    data: i32,
}

fn main() {
    let tasks_queue = retrieve_tasks();

    tasks_queue.into_iter()
                .map(|(t, o)| compute_operation(t.data, o)) 
                .for_each(|res| {
                    println!("Task give {}", res.unwrap_or("Error".to_string()))
                });
}

fn compute_operation(data: i32, operation: Operations) -> Result<String, OperationsError> {
    
    return match operation {
        Operations::ADD => add_operation(data),
        
        Operations::MUL => mul_operation(data),
        
        Operations::DIV => div_operation(data),
    }
}

fn retrieve_tasks() -> Vec< (Task, Operations) > {
    vec![
        ( Task{ id: 1, data: 1}, Operations::ADD ),
        ( Task{ id: 2, data: 2}, Operations::MUL ),
        ( Task{ id: 3, data: 2}, Operations::MUL ),
        ( Task{ id: 4, data: 30}, Operations::DIV ),
    ]
}

fn add_operation(data: i32) -> Result<String, OperationsError> {
    let compute = data + 10;
    return Ok(String::from(compute.to_string()))
}

fn mul_operation(data: i32) -> Result<String, OperationsError> {
    let compute = data * 10;
    return Ok(String::from(compute.to_string()))
}

fn div_operation(data: i32) -> Result<String, OperationsError> {
    return Err(OperationsError::OVERFLOW)
}