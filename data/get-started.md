## Reliable

---

## First code

---

## Tasks queue processing

![task queue schema](assets/img/tasks_processing_schema.jpg)

---

## Task Modelisation

```rust
struct Task {
    id: i64,
    data: i32,
}

enum Operations {
    ADD,
    MUL,
    DIV
}

==> ( Task{ id: 1, data: 1}, Operations::ADD )
```

---

## Construct tasks queue

```rust

fn retrieve_tasks() -> Vec< (Task, Operations) > {
    
    vec![
        ( Task{ id: 1, data: 1}, Operations::ADD ),
        ( Task{ id: 2, data: 2}, Operations::MUL ),
        ( Task{ id: 3, data: 2}, Operations::MUL ),
        ( Task{ id: 4, data: 30}, Operations::DIV ),
    ]
}

```

---

## Pattern matching for operations

```rust
fn compute_operation(data: i32, operation: Operations)
    -> Result<String, OperationsError> {
    
    match operation {
        Operations::ADD => add_operation(data),
        
        Operations::MUL => mul_operation(data),
        
        Operations::DIV => div_operation(data),
    }
}
```

---

## Pattern matching for operations

```rust
fn add_operation(data: i32) -> Result<String, OperationsError> {
    let compute = data + 10;
    Ok(String::from(compute.to_string()))
}


fn mul_operation(data: i32) -> Result<String, OperationsError> {
    let compute = data * 10;
    Ok(String::from(compute.to_string()))
}


fn div_operation(data: i32) -> Result<String, OperationsError> {
    Err(OperationsError::OVERFLOW)
}
```

---

## Errors modelisation

```rust
enum OperationsError {
    UNKNOW_OPERATION,
    OVERFLOW,
    PARSING,
}
```

---

## main

```rust
fn main() {
  let tasks_queue = retrieve_tasks();

  tasks_queue.into_iter()
             .map(|(t, o)|   compute_operation(t.data, o))
             .map(|res|      res.unwrap_or(String::from("Error")))
             .for_each(|res| println!("Task give {}", res));
}
```

[full code](https://github.com/loganmzz/rust-presentation-introduction/blob/master/examples/src/bin/getting_startv3.rs)

---

## Structures (`struct`)

```rust
#[derive(Clone, Debug)]
struct Data(i32, i32);

struct Task {
    id: i64,
    data: Data,
}

impl Task {
    fn new(id: i64, data: Data) -> Task {
        Task { id, data }
    }

    fn data(self) -> Data {
        self.data
    }
}
```

[examples_struct.rs](https://github.com/loganmzz/rust-presentation-introduction/blob/master/examples/src/bin/examples_struct.rs)

Note:
* `struct` => data container
* `impl` => "method"
* `self` => instance function
* else  => "associated" (static) function

---

## Type set (`enum`)

```rust
enum Person {
    Anonymous,
    Natural { last_name: String, first_name: String },
    Company { name: String, owners: Vec<Person> },
    Association(String),
}

impl Person {
   fn display(self) {
      match self {
         Person::Anonymous => println!("I am no one"),
         Person::Natural { last_name, first_name } =>
            println!("Hello, my name is {} {}", first_name, last_name),
         Person::Company { name, .. } => println!("$$$ {} $$$", name),
         Person::Association(nom) => println!("❤❤❤ {} ❤❤❤", nom)
      }
   }
}
```

[examples_enum.rs](https://github.com/loganmzz/rust-presentation-introduction/blob/master/examples/src/bin/examples_enum.rs)

Note:
* Finite variant set
* Variants are `struct`-like

---

## Contract (`trait`)

```rust
impl std::ops::Add for Data {
    type Output = Data;

    fn add(self, rhs: Data) -> Data {
        Data(self.0 + rhs.0, self.1 + rhs.1)
    }
}

println!("{:?}", Data(1,2) + Data(4,8));
```

[examples_trait.rs](https://github.com/loganmzz/rust-presentation-introduction/blob/master/examples/src/bin/examples_trait.rs)

Note:
* (= Java) no attributes
* (= Java) default method implementation
* (~ Java) inheritance (_composition over inheritance_)
* (+ Java) static methods
* (+ Java) operator overriding

---

## Generic

```rust
struct Km; struct Mi;
struct Distance<T>(u32, T);

trait Adaptable<T> {
   fn adapt(self) -> T;
}

impl Adaptable<Distance<Mi>> for Distance<Km> {
   fn adapt(self) -> Distance<Mi> {
      Distance(self.0 * 13 / 21, Mi)
   }
}

impl Adaptable<Distance<Km>> for Distance<Mi> {
   fn adapt(self) -> Distance<Km> {
      Distance(self.0 * 21 / 13, Km)
   }
}
```

[examples_generic.rs](https://github.com/loganmzz/rust-presentation-introduction/blob/master/examples/src/bin/examples_generic.rs)

Note:
* C++-like template
* Static dispatch*

* (- Java) no inheritance => no variance
* (+ Java) advanced inference
* (+ Java) no reification => static dispatch
* (+ Java) many implementations

---

## Generic

```rust
#[derive(Clone, Debug)]
struct Hours;
struct Speed<D, T=Hours>(Distance<D>, T);

impl< DT , DS,S >  Adaptable<Speed<DT,S>>  for  Speed<DS,S>  where
   Distance<DS>: Adaptable<Distance<DT>>,
   S: Clone {
        fn adapt(self) -> Speed<DT,S> {
            Speed(self.0.adapt(), self.1.clone())
        }
}
```

[examples_generic.rs](https://github.com/loganmzz/rust-presentation-introduction/blob/master/examples/src/bin/examples_generic.rs)
