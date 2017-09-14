## Cargo workflow
![cargo logo](assets/img/cargo_logo.png)

> Cargo is the Rust primary workflow tool.

---

## Cargo goals

1. Conventions for all Rust projects

2. Fetches and builds your project’s dependencies

3. Always get a **repeatable build**

---

## Project layout

```
├── Cargo.lock
├── Cargo.toml
├── benches
│   └── micros_bench.rs
│
├── examples
│   └── example.rs
│
├── src
│   ├── lib.rs  // --lib
│   └── main.rs // --bin
│
└── tests
    └── integration-tests.rs
```

---

### [Crates.io](https://crates.io/) 
<video muted loop onclick="this.play();">
    <source data-src="assets/img/cargo.mp4" type="video/mp4" />
</video>

---

### [Docs.rs](https://docs.rs/)
<video muted loop onclick="this.play();">
    <source data-src="assets/img/docs_rs.mp4" type="video/mp4" />
</video>

Note:
Automatically builds crates' documentation released on [crates.io](https://crates.io/)

---

## Repeatable build

> The first time a build succeeds, Cargo emits a `Cargo.lock` file, which contains a manifest of precisely which source code was used in the build. 

---

## Shared Dependencies

Share dependencies between crates if they depend on the same major version