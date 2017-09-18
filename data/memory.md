## Memory & paralellism

* Stack by default
* Optimisations
    * Move to stack
    * Move to heap

Note:
* (= Java) Escape analysis
* Smart pointers

---

## Heap ([Box](https://doc.rust-lang.org/std/boxed/struct.Box.html))

```rust
let stack = Person::new("stack");
println!("stack={:?}", stack.name());

let boxed = Box::new(Person::new("boxed"));
println!("boxed={:?}", boxed.name());
```

```rust
fn consume(boxed: Box<Person>) {
    // Free box here
} 
consume(boxed);
consume(boxed); // use of moved value: `boxed`
```
<!-- .element class="fragment" -->

```rust
// Free stack here
```
<!-- .element class="fragment" -->

[examples_box.rs](https://github.com/loganmzz/rust-presentation-introduction/blob/master/examples/src/bin/examples_box.rs)

Note:
Boxes works transparently as default reference

---

## Shared reference ([Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html))

```rust
use std::sync::Arc;

let stack = Person::new("stack");
println!("stack={:?}", stack.name());

let refc0 = Arc::new(Person::new("refctr"));
println!("refc0={:?}", refc0.name());
```

```rust
fn consume(rfctr: Arc<Person>) { ... }
consume(rfctr.clone());
consume(rfctr.clone());
```
<!-- .element class="fragment" -->

```rust
// Free stack & rfctr here
```
<!-- .element class="fragment" -->

[examples_arc.rs](https://github.com/loganmzz/rust-presentation-introduction/blob/master/examples/src/bin/examples_arc.rs)

Note:
* **A**tomic **R**eference **C**ounter
* `std::sync::Arc` => associated fonctions VS target reference methods

---

## Light process (`thread`)

```rust
fn computation() -> i64 {
    42 * 314
}

let handle = thread::spawn(computation);


let result = handle.join().expect("Error during computation");
println!("Result: {}", result);

```

Note:
`examples-thread.rs`

---

## Transfer ([Send](https://doc.rust-lang.org/std/marker/trait.Send.html))
## Share ([Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html))

```rust
let reference = Arc::new(String::from("A shared string"));

let handles = (0..2)
           .map(|_| reference.clone())
           .map(|shared|
                spawn(move ||
                    println!("{:?} => {:?}", current().id(), shared))
            )
            .collect();
                
for handle in handles {
    handle.join().unwrap();
}
```

Note:
* `examples-thread-send-async.rs`
* Automatic implementations based on attributes

---

## Data exchange ([channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html))

`1 Receiver` and `1..N Sender`

```rust
let (tx, rx) = mpsc::channel();
let tx2 = tx.clone();

thread::spawn(move || {
    let mut scores = vec![1, 2];
    tx.send(scores);
});
thread::spawn(move || tx2.send(vec![3, 4]));

for scores in rx {
    println!("Received : {:?}", scores);
}
```

![go_die](assets/img/gopher_ahah.png)
<!-- .element class="fragment fade-up" -->

<!-- .element style="margin-top: -20px" -->

---

## Ownership messaging (safety)

<pre><code data-trim data-noescape class="rust">
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let mut scores = vec![2, 4];
    tx.send(scores);
    <span class="fragment highlight-mark">scores.push(125);</span>
});
</code></pre>

<pre><code data-trim data-noescape class="rust"> 
error[E0382]: use of moved value: `scores`
  --> main.rs:13:3
12 | 		tx.send(scores);
   | 		        <span class="fragment highlight-mark">- value moved here</span>
13 | 		scores.push(125);
   | 		<span class="fragment highlight-mark">^ value used here after move</span>
</code></pre>
<!-- .element class="fragment" -->

---

### Concurrency without _data races_
