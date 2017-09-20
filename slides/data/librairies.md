## Librairies

| Asynchronous | Web | Serialization | Miscellaneous |
| ---------- | --- | ------------- | ------ |
| Mio | Iron | Serde | Piston |
| Tokio | Rocket | Nom | Leaf |
| Rayon | Nickel | Diesel | Chrono |
| Crossbeam | Gotham | ProtoBuf | |

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
