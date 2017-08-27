## Types

* Structures
* Énumérations
* Traits
* Génériques

---

## Structures (`struct`)

* _Unit_

* Classique

* Tuples

Note: 
`examples-struct.rs`

---

## Énumérations (`enum`)

* Ensemble fini de variantes
* Les variantes sont comme des `struct`

Note:
`examples-types-enum.rs`

---

## Traits (`trait`)

* Interfaces à la Java
* Implémenter par l'auteur du trait ou type cible
_Orphan rule_
* Surcharge d'opérateur

Note:
* (= Java) pas d'attributs
* (= Java) implémentations par défaut
* (~ Java) héritage (_composition over inheritance_)
* (+ Java) méthodes statiques
* (+ Java) surcharge d'opérateur
* `examples-types-trait.rs`

---

## Generics

* _Template_ à la C++
* _static dispatch_*
* Implémentations générique (`impl<...>`)
* Contraintes complexes (`where`)

Note:
* (- Java) pas héritage => pas variance
* (+ Java) contraintes négatives
* (+ Java) inférence avancée
* (+ Java) pas réification => static dispatch
* (+ Java) plusieurs implémentations
* (+ Java) types par défaut
* `examples-types-generic.rs`
