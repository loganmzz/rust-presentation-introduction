## Macros

<table class="no-border center-mid">
<tr class="fragment"> <td>**functions**</td> <td>abstract over</td> <td>_variables_</td> </tr>
<tr class="fragment"> <td>**generics**</td> <td>...</td> <td>_types_</td>
<tr class="fragment"> <td>**macros**</td> <td>...</td> <td>_syntax tree_</td>
</table>

Note:
* AST-based ⇒ both call and definition must be parsable 

---

## Defining a macro

``` rust
macro_rules! example {
    (x => $e: expr) => (println!("mode X: {}", $e));
    
    (y => $e: expr) => (println!("mode Y: {}", $e));
}

example!(x => 2 + 2); //get mode X: 4
```

Note:
* rules (left-part) match against token tree
* expansion (right-part) generates other tokens (not source code) !
* valid delimiter: `()`, `{}` or `[]` (for invocation, rules and expansion)

---

## **A**bstract **S**yntax **T**ree

<pre><code data-trim data-noescape class="text">
macro_rules! new_macro {
    <i class="h3">(</i><i class="h1">x => $e: expr</i><i class="h3">)</i> => <i class="h3">(</i><i class="h2">println!("mode X: {}", $e)</i><i class="h3">)</i>;
}
new_macro!<i class="h3">(</i><i class="h4">x => 2 + 2</i><i class="h3">)</i>
</pre></code>

<i class="h1">rules</i> consists in pattern-matching tokens 

<i class="h2">expansion</i> generates new tokens (not code !)

<i class="h3">delimiter</i> can be `()`, `{}`, `[]` for <i class="h4">invocation</i>, <i class="h1">rules</i>, and <i class="h2">expansion</i>

---

## Access AST 

| | |
| --- | --- |
| `block` | |
| `expr` | ⇒ expressions
| `ident` | ⇒ identifiers (variable/function names)
| `item` | ⇒ component of a crate (i.e. global definitions)
| `pat` | ⇒ pattern
| `path` | ⇒ (e.g. `::std::fmt`)
| `stmt` | ⇒ statement
| `tt` | ⇒ token tree
| `ty` | ⇒ type
| `meta` | ⇒ attribute content (i.e. `#[...]`)
<!-- .element class="headless compact" -->

---

## Pragmatic example

``` rust
macro_rules! find_min {
    ($x:expr) => ($x); // only one element
    
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

println!("{}", find_min!(5, 8, 9, 10));
```
Note:
* `$(...)*` repetition operator
* `$(...),+` match one or more expression
