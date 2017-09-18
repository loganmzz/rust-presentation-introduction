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

---

### Librairies - Serialisation

How we can parse _Wavefront.obj_ file ?

```rust
        #File exported by ZBrush version 3.5
        #www.zbrush.com
        mtllib head.mtl
        usemtl defaultMat
        v 0.02866628 0.03189809 -0.18487546
        v 0.02733818 -0.00265587 -0.18358554
        v 0.1779453 -0.24393026 -0.19571779
        v 0.1453844 -0.23073842 -0.19898235
        ...
        vt 0.70332 0.80831
        vt 0.67975 0.82279
        vt 0.67272 0.79891
        ...
```

---

### use [nom](https://github.com/Geal/nom) !

Parser combinator framework with zero copy

```rust
named!(pub four_float_opt_4th< &[u8], (f32, f32, f32, Option<f32>)>, 
    sp!(
        tuple!(float, float, float, 
        opt!(float))
    )
);

named!(vertices_geometry<&[u8], Face>, map!(
    sp!( 
        delimited!(
            tag!("v"), 
            four_float_opt_4th, nom::eol
        )
    ), 
    |(x, y, z, w)| Face::new(x, y, z, w)
));
```