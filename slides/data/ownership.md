
<!-- .slide: data-background="assets/img/safety.gif" -->

---

<p>No safety = <span>mutable data</span><!-- .element: class="fragment" --><span> + more one ref on it</span><!-- .element: class="fragment" -->

---

## Garbage collector

![cargo logo](assets/img/gc.png)

_Can't prevent_: data races, iterator invalidation,...</p> <!-- .element class="fragment" -->

---

## Java _Runtime errors_
```
Exception in thread "main" java.lang.NullPointerException

... 10ms after

Exception in thread "main" java.lang.ArrayIndexOutOfBoundsException
```

---

## C/C++ _Runtime errors_

```
$ ./client_demo

Segmentation fault
```

---

<!-- .slide: data-background="assets/img/debuger_rescue.gif" -->


---

<!-- .slide: data-background="assets/img/dont_want.gif" -->

---

## Rust solution

> Rust _avoids_ the need for _GC_ through:

* ownership 
* borrowing
* lifetime

**Check at compile time.**
<!-- .element: class="fragment" --> 

---

## Ownership

Ensure **only one** active _binding_ to _allocated memory at a time_

_✔️ eliminates double frees, use after free..._  <!-- .element: class="fragment" -->

---

## Ownership

<pre><code data-trim data-noescape class="rust"> 
fn generate_events() {
    let <span class="fragment highlight-mark">events</span> = vec![...];
    transform_events(events); //take events ownership
}

fn transform_events(<span class="fragment highlight-mark">events</span>: Vec<Event>) {
    // Ownership of events transfered in
    // transform_event scope 
} <span class="fragment" style="color:red;font-size:larger;">◀️ free memory allocation events</span>
</code></pre> 


---

## Ownership

![vec_ownership](assets/img/ownership_events_example.svg) <!-- .element: width="70%" style="background-color:white;" -->

---

## Ownership

<pre><code data-trim data-noescape class="rust"> 
fn generate_events() {
    let events = vec![...];

    transform_events(events);
    <span class="fragment highlight-mark">transform_events(events);</span> 
}

fn transform_events(events: Vec<Event>) { /* ... */ }
</code></pre> 

---

<pre><code data-trim data-noescape class="rust"> 
error[E0382]: use of moved value: `events`
 --> src/move_example.rs:4:19
  |
3 |   transform_events(events);
  |                   <span class="fragment highlight-mark">------ value moved here</span>
4 |   transform_events(events);
  |                   <span class="fragment highlight-mark">^^^^^^ value used here after move</span>
</code></pre> 
<!-- .element class="fragment" -->


---

## Borrowing

> What if, we **borrow** the resource instead?

---

## Borrowing &T

Many reader, no writers

``` rust
fn read_events() {
    let treasures = get_treasures(...); // take ownership

    get_value_treasures(&treasures); //reader
    get_date_treasures(&treasures); //reader
}

fn get_value_treasures(treasures: &Vec<Treasure>) {
    // Can't modify the treasures here
}
```

---

## Borrowing &mut T

Exactly _one mutable_ reference

``` rust
let mut treasures = get_treasures(...); // take ownership

let t1 = &mut treasures;
modify_values_treasures(t1);

let t2 = &mut treasures;
modify_owner_treasures(t2);

fn get_value_treasures(treasures: &mut Vec<Treasures>) {
    // Increase the number of golds
}
```

---

## Borrowing &mut T

<pre><code data-trim data-noescape class="rust">
error[E0499]: cannot borrow `treasures` as mutable more than 
once at a time
 --> src/main.rs:7:19
  |
4 |     let t1 = &mut treasures;
  |                  <span class="fragment highlight-mark">--------- first mutable borrow occurs here</span> 
  |                              
...
7 |     let t2 = &mut treasures; 
  |                   <span class="fragment highlight-mark">^^^^^^^^^ second mutable borrow occurs here</span> 
  |                             
8 |     get_value_treasures(t2);
9 | }
  | - first borrow ends here
</code></pre>

---

## Borrowing look like

> Compile time _read-write lock_ on data (not the code)

---

## To conclude

<p>Rust code is not just _fast_ because of _no GC_</p>  <!-- .element: class="fragment" -->

**it's more safety !!** <!-- .element: class="fragment big" -->
