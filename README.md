# Tomorrow I stop coffee, I get to crab !

Hosted slides are available at : [https://loganmzz.github.io/rust-presentation-introduction/](https://loganmzz.github.io/rust-presentation-introduction/)

Introduction to Rust language and its ecosystem. Goal is to ease starting with Rust for people with knowledge of an high-level language as Java.

* Language (3 foundations)
  * Productivity: functional-style, no manual memory management, strong and infered typing
  * Performance: native compilation through LLVM, ∅GC, low-level language
  * Safety : memory safety, no concurrent access
  * Features
    * struct / trait / generics
    * ownership / borrowing
    * memory guarantees : Box / Arc
    * macro
* Ecosystem :
  * IDE, tests, debugger, RLS
  * Build et package manager (Cargo)
  * Librairies (Rayon, Rocket, Nom)

## Rust installation

Follows instructions available at https://www.rustup.rs/.

## Cross-compilation

In order to cross-compile, you need both a Rust toolchain for the target platform and a "linker" for this platform.

Following instructions help in order to setup a Debian-based environment to target ARMv7.

## ARMv7

First, you need to install a 32-bits compiler as ARMv7 is a 32-bits architecture:

```
apt-get update && apt-get install libc6-dev-i386
```

Next, install cross-compiling tools

```
apt-get update && apt-get install gcc-arm-linux-gnueabihf
```

Then, install target to Rust toolchain

```
rustup target add armv7-unknown-linux-gnueabihf
```

Finally, specify linker Cargo should use

```
cat >>~/.cargo/config <<EOF
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF
```

In order to generate ARMv7 binaries:

```
cargo build --target armv7-unknown-linux-gnueabihf
```

Optionally, you can omit `--target` flag by adding the following section to your `~/.cargo/config`:

```
[build]
target = "armv7-unknown-linux-gnueabihf"
```

