## Librairies

| Asynchronous | Web | Serialization | Miscellaneous |
| ---------- | --- | ------------- | ------ |
| [Mio](https://crates.io/crates/mio) | [Iron](https://crates.io/crates/iron) | [Serde](https://crates.io/crates/serde) | [Piston](https://crates.io/crates/piston) |
| [Tokio](https://crates.io/crates/tokio-core) | [Rocket](https://crates.io/crates/rocket) | [Nom](https://crates.io/crates/nom) | [Leaf](https://crates.io/crates/leaf) |
| [Rayon](https://crates.io/crates/rayon) | [Nickel](https://crates.io/crates/nickel) | [Diesel](https://crates.io/crates/diesel) | [Chrono](https://crates.io/crates/chrono) |
| [Crossbeam](https://crates.io/crates/crossbeam) | [Gotham](https://crates.io/crates/gotham) | [ProtoBuf](https://crates.io/crates/protobuf) | |

---

### Librairies - Asynchrone

[Rayon](https://github.com/nikomatsakis/rayon)

```rust
extern crate rayon;

fn mul_par(lhs: &Matrix, rhs: &Matrix,
           index: usize, data: &mut [u32]) {
   // Small enough: sequential
   if data.len() < threhsold {
      mul_seq(lhs, rhs, index, data);
   } else {
   // Too big: parallel
      let split = data.len() / 2;
      let (head, tail) = data.split_at_mut(split);
      rayon::join(|| mul_par(lhs, rhs, index        , head),
                  || mul_par(lhs, rhs, index + split, tail)
      );
   }
}
```

---

### Librairies - Web

🚀 [Rocket](https://rocket.rs/)

```rust
#[get("/")]
fn index() -> Json {
    Json(
        json!({
            "subject": "rust",
            "hall": 3,
        })
    )
}

fn main() {
    rocket::ignite()
            .mount("/", routes![index])
            .launch();
}
```
