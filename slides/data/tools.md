## Ecosystème

---

## [IDEs](https://forge.rust-lang.org/ides.html)

* [Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=saviorisdead.RustyCode)
* [IntelliJ IDEA](https://intellij-rust.github.io/)
* [Eclipse](https://github.com/RustDT/RustDT)
* [Vim](https://github.com/rust-lang/rust.vim)
* ...

[Racer](https://github.com/racer-rust/racer) provide code completion

---

## Rust [tooling](https://gist.github.com/nrc/a3bbf6dd1b14ce57f18c)

> Rust can work with pretty much all C/C++ agnostic tools

`kcov, gdb, lldb, perf...`

---

## [Travis CI](https://docs.travis-ci.com/user/languages/rust/)

``` yaml
language: rust
rust:
  - stable
  - beta
  - nightly
matrix:
  allow_failures:
    - rust: nightly
```