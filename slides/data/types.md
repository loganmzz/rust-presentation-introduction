## Types

* Structures
* Enumerations
* Traits
* Generics

---

## Structures (`struct`)

* _Unit_

* Classic

* Tuples

Note: 
`examples-struct.rs`

---

## Enumerations (`enum`)

* Finite variant set
* Variants are `struct`-like

Note:
`examples-types-enum.rs`

---

## Traits (`trait`)

* Java-like interfaces
* Implemented by author from trait or target type
_Orphan rule_
* Operator overriding

Note:
* (= Java) no attributes
* (= Java) default method implementation
* (~ Java) inheritance (_composition over inheritance_)
* (+ Java) static methods
* (+ Java) operator overriding
* `examples-types-trait.rs`

---

## Generics

* C++-like template
* Static dispatch*
* Generic implementation (`impl<...>`)
* Complex constraints (`where`)

Note:
* (- Java) no inheritance => no variance
* (+ Java) negative constraints
* (+ Java) advanced inference
* (+ Java) no reification => static dispatch
* (+ Java) many implementations
* (+ Java) default type parameter
* `examples-types-generic.rs`
