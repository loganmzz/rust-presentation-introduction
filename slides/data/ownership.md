
<!-- .slide: data-background="/assets/img/safety.gif" -->

---

## Memory model in Java

#### <span style="color:orange">Garbage collector</span>

![cargo logo](/assets/img/gc.png)

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

``` rust

fn generate_events() {
    let events = vec![...];
    transform_events(events); //take events ownership
}

fn transform_events(events: Vec<Event>) {
    // Ownership of events transfered in
    // transform_event scope 
} 

```

---

## Ownership

#### free memory

When <span style="color:red">`events[]`</span> <span style="color:orange">goes out of scope</span> at the end of `transform_events`, 

<p>Rust will <span style="color:orange">reclaim</span> the <span style="color:orange">memory</span> allocated at the <span style="color:orange">end of the scope</span>.</p>
 <!-- .element: class="fragment" -->

---

## Ownership

``` rust

fn generate_events() {
    let events = vec![...];

    transform_events(events);
    transform_events(events);
}

fn transform_events(events: Vec<Event>) {
    //...
}

```

``` rust
error[E0382]: use of moved value: `events`
 --> src/move_example.rs:4:19
  |
3 |   transform_events(events);
  |                   ------ value moved here
4 |   transform_events(events);
  |                   ^^^^^^ value used here after move
  |
```

---

## Ownership

![vec_ownership](/assets/img/vec_ownership.png)

---

## Ownership

> Ensure <span style="color:red">only one</span> active <span style="color:orange">binding</span> to  
> <span style="color:orange">allocated memory at a time</span>

>  <span style="color:green">✔️ eliminates data race</span>  <!-- .element: class="fragment" -->

---

## Borrowing

> What if, we <span style="color:red">borrow</span> the resource instead?

---

Not safety = <span>mutable data</span><!-- .element: class="fragment" --><span> + more one ref on it</span><!-- .element: class="fragment" -->

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

Exactly one mutable reference

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

```
error[E0499]: cannot borrow `treasures` as mutable more than 
once at a time
 --> src/main.rs:7:19
  |
4 |     let t1 = &mut treasures;
  |                   --------- first mutable borrow 
  |                              occurs here
...
7 |     let t2 = &mut treasures; 
  |                   ^^^^^^^^^ second mutable 
  |                             borrow occurs here
8 |     get_value_treasures(t2);
9 | }
  | - first borrow ends here
```

---

## ownership look like

> Compile time <span style="color:orange">read-write lock</span> on data (not the code)

---

## To Conclude

<p> Rust code is not just <span style="color:orange">fast</span> because of <span style="color:orange">no GC</span></p>  <!-- .element: class="fragment" -->

<p> it's <span style="color:red">more safety !!</span></p> <!-- .element: class="fragment" -->