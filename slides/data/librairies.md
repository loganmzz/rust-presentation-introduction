## Librairies

| Asynchronous | Web | Serialization | Miscellaneous |
| ---------- | --- | ------------- | ------ |
| Mio | Iron | Serde | Piston |
| Tokio | Rocket | Nom | Leaf |
| Rayon | Nickel | Diesel | Chrono |
| Crossbeam | | ProtoBuf | |

---

#### Librairies - Asynchrone

* Rayon

```rust
```

---

### Librairies - Web

🚀 [Rocket](https://rocket.rs/)

```rust
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
            .mount("/", routes![index])
            .launch();
}
```

---

### Librairies - Serialisation

How we can parse Wavefront .obj file ?

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

```rust
named!(vertices_geometry<&[u8], Face>, map!(
    sp!( delimited!(tag!("v"), four_float_opt_4th, nom::eol)),
    |(x, y, z, w)| Face::new(x, y, z, w)
));


named!(texture_coords<&[u8], Coord>, map!(
    sp!( delimited!(tag!("vt"), triple_float_opt_3rd, nom::eol)),
    |(u, v, w))| TextureCoord::new(u, v, w)
));


named!(pub four_float_opt_4th< &[u8], (f32, f32, f32, Option<f32>)>, 
    sp!(tuple!(float, float, float, opt!(float)))
);
```