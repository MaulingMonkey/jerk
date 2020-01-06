# **jerk**-build: **J**ava **E**mbedding **R**ust **K**it - [build.rs] / [metabuild] script

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/jerk.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/jerk)
![unsafe: no](https://img.shields.io/badge/unsafe-no-green.svg)
![rust: 1.36.0+](https://img.shields.io/badge/rust-1.36.0%2B-green.svg)
[![License](https://img.shields.io/crates/l/jerk-build.svg)](https://github.com/MaulingMonkey/jerk-build)

A library to compile/embed Java alongside a Rust library/application.
Similar to [cc], but for Java.
This is **not** an official project of Google, Oracle, Sun Microsystems, or anyone else.

## Quick Start:  [build.rs]

Add the following to your executable's Cargo.toml:

```toml
[build-dependencies]
jerk-build = "0.2"
```

And the following to your [build.rs]:
```no_run
fn main() {
    jerk_build::metabuild();
}
```

# Quick Start:  [metabuild] (nightly only)

Add the following to your executable's Cargo.toml:

```toml
cargo-features = ["metabuild"]

[package]
metabuild = ["jerk_build"]

[build-dependencies]
jerk-build = "0.2"
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.



[cc]:                       https://crates.io/crates/cc
[build.rs]:             https://doc.rust-lang.org/cargo/reference/build-scripts.html
[metabuild]:            https://github.com/rust-lang/rfcs/blob/master/text/2196-metabuild.md
