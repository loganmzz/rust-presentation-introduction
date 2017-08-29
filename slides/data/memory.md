## Mémoire et parallélisme

* Sur la pile (par défaut)
* Optimisations
    * Transfert vers la pile
    * Transfert vers le tas

Note:
* (= Java) _Escape analysis_
* _Smart pointers_

---

## `Drop`

* `finalize()` de Java
* Déallocation manuelle `drop()`

```rust
struct Personne(String);
impl Drop for Personne {
    fn drop(&mut self) {
        println!("Au revoir {}", self.0);
    }
}

let richard = Personne(String::from("RICHARD"));
drop(richard);
```

Note:
* `std::mem::drop` => _ownership_
`examples-memory-drop.rs`

---

## Tas (`Box`)

* Type récursif

```rust
enum Liste<T> {
    Nil,
    Cons(T, Box<Liste<T>>),
}
```

* Taille inconnue

```rust
trait Foo: Debug {}
fn bar(foo: Box<Foo>) {
    println!("{:?}", foo);
}
```

Note:
`examples-memory-box.rs`

---

## Pointeur partagé (`Arc`)

```rust
fn dis_bonjour(personne: Arc<Personne>) {
    println!("Bonjour {}", persone.0);
}

fn essaie_de_dire_bonjour(personne: Weak<Personne>) {
    personne.upgrade().map_or_else(
        | | println!("La personne est partie ..."),
        |p| dis_bonjour(p)
    );
}

let robert = Arc::from(Personne(String::from("ROBERT")));
let faible = Arc::downgrade(&robert);
dis_bonjour(robert.clone());
essaie_de_dire_bonjour(faible.clone());
drop(robert);
essaie_de_dire_bonjour(faible);
```

Note:
* **A**tomic **R**eference **C**Counter
* `std::sync::Arc` => fonctions associées VS méthodes référence ciblée
* `std::sync::Weak`
    * pas d'accès aux méthodes référence ciblée
    * casser les cycles
* `examples-memory-arc.rs`


---

## Processus léger (`thread`)

```rust
fn calcul() -> i64 {
    42 * 314
}

let handle = thread::spawn(calcul);
let resultat = handle.join().expect("Erreur durant le calclul");
println!("Résultat: {}", resultat);
```

Note:
`examples-thread.rs`

---

## Tranfert (`Send`) et Partage (`Sync`)

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
* Automatique si tous les composant sont `Send` ou `Sync`

---

## Channel

```rust
fn main() {

	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let mut scores = vec![2, 4];
        tx.send(scores);
	});

	let scores: Vec<i32> = rx.recv().unwrap();
	println!("{:?}", scores);
}
```

![go_die](/assets/img/gopher_ahah.png) <!-- .element style="background:none; border:none; box-shadow:none;" -->

---

## Ownership messaging (safety)

```rust
fn main() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let mut scores = vec![2, 4];
        tx.send(scores);
        scores.push(125);
	});

	let scores: Vec<i32> = rx.recv().unwrap();
	println!("{:?}", scores);
}
```

```rust
error[E0382]: use of moved value: `scores`
  --> main.rs:13:3
12 | 		tx.send(scores);
   | 		        - value moved here
13 | 		scores.push(125);
   | 		^ value used here after move
```