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

## Tas (`Box`)

* Recursive types

```rust
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}
```

* Unknown size

```rust
trait Foo: Debug {}
fn bar(foo: Box<Foo>) {
    println!("{:?}", foo);
}
```

Note:
`examples-memory-box.rs`

---

## Shared reference (`Arc`)

```rust
fn say_hello(invidual: Arc<Person>) {
    println!("Hello {}", person.0);
}

fn try_say_hello(person: Weak<Person>) {
    person.upgrade().map_or_else(
        | | println!("Person is gone ..."),
        |p| say_hello(p)
    );
}

let robert = Arc::from(Person(String::from("ROBERT")));
let weak = Arc::downgrade(&robert);
say_hello(robert.clone());
try_say_hello(weak.clone());
drop(robert);
try_say_hello(faible);
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

## Transfer (`Send`)
## Share (`Sync`)

```rust
let reference = Arc::new(String::from("A shared string"));
let handles: Vec<_> = (0..2).map(|_| reference.clone())
                            .map(|shared|
   spawn(move || println!("{:?} => {:?}", current().id(), shared))
                            )
                            .collect()
for handle in handles {
    handle.join().unwrap();
}
```

Note:
* `examples-thread-send-async.rs`
* Automatic implementations based on attributes

---

## Data exchange (`channel`)

* 1 consumer (`Receiver`)
* 1..N producers (`Sender`)

```rust
let (tx1, rx) = channel();
let tx2 = tx1.clone();

spawn(move || tx1.send(vec![1, 2, 3]).unwrap());
spawn(move || tx2.send(vec![4, 5, 6]).unwrap());

for scores in rx {
    println!("Received : {:?}", scores);
}
```

![go_die](/assets/img/gopher_ahah.png) <!-- .element class="fragment fade-up" style="background:none; border:none; box-shadow:none;" -->

---

## Ownership messaging (safety)

<pre><code data-trim data-noescape class="rust">
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let mut scores = vec![2, 4];
    tx.send(scores);
    <span class="fragment highlight-mark">scores.push(125);</span>
});

let scores: Vec<i32> = rx.recv().unwrap();
println!("{:?}", scores);
</code></pre>

<pre><code data-trim data-noescape class="rust"> 
error[E0382]: use of moved value: `scores`
  --> main.rs:13:3
12 | 		tx.send(scores);
   | 		        <span class="fragment highlight-mark">- value moved here</span>
13 | 		scores.push(125);
   | 		^ <span class="fragment highlight-mark">value used here after move</span>
</code></pre>
<!-- .element class="fragment" -->

---

### Concurrency without data races 
