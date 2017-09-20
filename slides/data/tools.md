## Ecosystem

---

## [IDE](https://forge.rust-lang.org/ides.html)

* VS Code ([by kalitaalexey](https://marketplace.visualstudio.com/items?itemName=kalitaalexey.vscode-rust), [by rust-lang](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust))
* [IntelliJ IDEA](https://intellij-rust.github.io/)
* [Eclipse](https://github.com/RustDT/RustDT)
* [Vim](https://github.com/rust-lang/rust.vim)
* ...

[Rust Language Server](https://github.com/rust-lang-nursery/rls): completion, refactor ...

Note:
* VS Code:
    * first one a fork from a dropped initial one
    * second one done recenlty in preview by official tool team

---

## Rust [tooling](https://gist.github.com/nrc/a3bbf6dd1b14ce57f18c)

> Rust can work with pretty much all C/C++ agnostic tools

`kcov`, `gdb`, `lldb`, `perf`...

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