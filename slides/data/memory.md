## Memory & paralellism

* Stack by default
* Optimisations
    * Move to stack
    * Move to heap

Note:
* (= Java) Escape analysis
* Smart pointers

---

## `Drop`

* Java-like `finalize()`
* Manual deallocation `drop()`

```rust
struct Person(String);
impl Drop for Person {
    fn drop(&mut self) {
        println!("Goodbye {}", self.0);
    }
}

let richard = Person(String::from("RICHARD"));
drop(richard);
```

Note:
* `std::mem::drop` => ownership
`examples-memory-drop.rs`

---

## Heap ([Box](https://doc.rust-lang.org/std/boxed/struct.Box.html))

Recursive types

```rust
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}
```

Unknown size

```rust
trait Foo: Debug {}
fn bar(foo: Box<Foo>) {
    println!("{:?}", foo);
}
```

Note:
`examples-memory-box.rs`

---

## Shared reference ([Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html))

```rust
let robert = Arc::from(Person(String::from("ROBERT")));
say_hello(robert.clone()); // Hello ROBERT

let weak = Arc::downgrade(&robert);
try_say_hello(weak.clone()); // Hello ROBERT

drop(robert);
try_say_hello(faible); // Person is gone
```

Note:
* **A**tomic **R**eference **C**ounter
* `std::sync::Arc` => associated fonctions VS target reference methods
* `std::sync::Weak`
    * no direct access to target reference methods
    * break reference cycles
* `examples-memory-arc.rs`


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

![go_die](/assets/img/gopher_ahah.png)
<!-- .element class="fragment fade-up" style="background:none; border:none; box-shadow:none;" -->

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
