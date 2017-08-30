
<!-- .slide: data-background="/assets/img/safety.gif" -->

---

<p>No safety = <span>mutable data</span><!-- .element: class="fragment" --><span> + more one ref on it</span><!-- .element: class="fragment" -->

---

## Garbage collector

![cargo logo](/assets/img/gc.png)

<p><span style="color:orange;">Can't prevent</span>: data races, iterator invalidation,...</p> <!-- .element class="fragment" -->

---

## Java <span style="color:orange">Runtime errors</span>
```
Exception in thread "main" java.lang.NullPointerException

... 10ms after

Exception in thread "main" java.lang.ArrayIndexOutOfBoundsException
```

---

## C/C++ <span style="color:orange">Runtime errors</span>

```
$ ./client_demo

Segmentation fault
```

---

<!-- .slide: data-background="/assets/img/debuger_rescue.gif" -->


---

<!-- .slide: data-background="/assets/img/dont_want.gif" -->

---

## Rust solution

> Rust <span style="color:orange">avoids</span> the need for <span style="color:orange">GC</span> through:

* ownership 
* borrowing
* lifetime

<span style="color:red">Check at compile time.</span>
<!-- .element: class="fragment" --> 

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

<pre><code data-trim data-noescape class="rust"> 
fn generate_events() {
    let events = vec![...];

    transform_events(events);
    <span class="fragment highlight-mark">transform_events(events);</span> 
}

fn transform_events(events: Vec<Event>) {
    //...
}
</code></pre> 


<pre><code data-trim data-noescape class="rust"> 
error[E0382]: use of moved value: `events`
 --> src/move_example.rs:4:19
  |
3 |   transform_events(events);
  |                   <span class="fragment highlight-mark">------ value moved here</span>
4 |   transform_events(events);
  |                   <span class="fragment highlight-mark">^^^^^^ value used here after move</span>
  |
</code></pre> 
<!-- .element class="fragment" -->

---

## Ownership

![vec_ownership](/assets/img/vec_ownership.png)

---

## Ownership

Ensure <span style="color:red">only one</span> active <span style="color:orange">binding</span> to  
<span style="color:orange">allocated memory at a time</span>

<p style="color:orange;font-size:larger">✔️ eliminates data race</span>  <!-- .element: class="fragment" -->

---

## Borrowing

> What if, we <span style="color:red">borrow</span> the resource instead?

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
    // Can't modify the events here
}
```

---

## Borrowing &mut T

Exactly <span style="color:orange">one mutable</span> reference

``` rust
fn main() {
    let mut treasures = get_treasures(...); // take ownership
    
    let t1 = &mut treasures;
    modify_values_treasures(t1);

    let t2 = &mut treasures; 
    modify_owner_treasures(t2);
}

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

## ownership look like

> Compile time <span style="color:orange">read-write lock</span> on data (not the code)

---

## To Conclude

<p> Rust code is not just <span style="color:orange">fast</span> because of <span style="color:orange">no GC</span></p>  <!-- .element: class="fragment" -->

<p style="color:red;font-size:175%"> it's more safety !!</p><!-- .element: class="fragment" -->