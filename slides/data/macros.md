## Macros

---

## Defining a macro

``` rust
macro_rules! example {
    (x => $e: expr) => (println!("mode X: {}", $e));
    
    (y => $e: expr) => (println!("mode Y: {}", $e));
}

...

example!(x => 2 + 2); //get mode X: 4
```

Note:
Series of `rules`, which are `pattern-matching` cases

---

## Access AST 

* block
* expr is used for expressions
* ident is used for variable/function names
* item
* pat (pattern)
* path
* stmt (statement)
* tt (token tree)
* ty (type)

---

## Pragmatic example

``` rust
macro_rules! find_min {
    ($x:expr) => ($x); // only one element
    
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn find_the_minus() {
    println!("{}", find_min!(5, 8, 9, 10));
}
```
Note:
* `$(...)*` repetition operator
* `$(...),+` match one or more expression