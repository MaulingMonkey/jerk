# **jerk**: **J**ava **E**mbedding **R**ust **K**it

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/jerk.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/jerk)
![unsafe: yes](https://img.shields.io/badge/unsafe-yes-yellow.svg)
![rust: 1.36.0+](https://img.shields.io/badge/rust-1.36.0%2B-green.svg)
[![License](https://img.shields.io/crates/l/jerk.svg)](https://github.com/MaulingMonkey/jerk)

Libraries to compile/embed/test Java alongside a Rust library/application.
Similar to [cc], but for Java.
This is **not** an official project of Google, Oracle, Sun Microsystems, or anyone else.

| Crate         | Badges | Notes |
| ------------- | ------ | ----- |
| [jerk](https://github.com/MaulingMonkey/jerk/tree/master/jerk)                | [![Crates.io](https://img.shields.io/crates/v/jerk.svg)](https://crates.io/crates/jerk)             [![Docs](https://docs.rs/jerk/badge.svg)](https://docs.rs/jerk/)              | Find Java paths, manage the JVM
| [jerk-build](https://github.com/MaulingMonkey/jerk/tree/master/jerk-build)    | [![Crates.io](https://img.shields.io/crates/v/jerk-build.svg)](https://crates.io/crates/jerk-build) [![Docs](https://docs.rs/jerk-build/badge.svg)](https://docs.rs/jerk-build/)  | Compile Java via [metabuild] / [build.rs] script
| [jerk-test](https://github.com/MaulingMonkey/jerk/tree/master/jerk-test)      | [![Crates.io](https://img.shields.io/crates/v/jerk-test.svg)](https://crates.io/crates/jerk-test)   [![Docs](https://docs.rs/jerk-test/badge.svg)](https://docs.rs/jerk-test/)    | Unit test Java from Rust

| Branch | Badges | Notes |
| ------ | ------ | ----- |
| [publish](https://github.com/MaulingMonkey/jerk/tree/publish) | [![Open issues](https://img.shields.io/github/issues-raw/MaulingMonkey/jerk.svg)](https://github.com/MaulingMonkey/jerk/issues) | Stable/published version
| [master](https://github.com/MaulingMonkey/jerk/tree/master)   | [![Build Status](https://travis-ci.org/MaulingMonkey/jerk.svg)](https://travis-ci.org/MaulingMonkey/jerk) | "Completed" stuff that hasn't been published.
| wip/*                                                         | | "Work In Progress" - incomplete, use at your own risk.
| dead/*                                                        | | Abandoned threads of work

## Goals

* Minimal dependencies
* Compile small amounts of Java before/alongside your Rust code via `javac`, `jar`, etc.
* **TODO?:** Compile small amounts of Kotlin before/alongside your Rust code via [`kotlinc`] etc.
* **TODO?:** Compile small amounts of whatever JVM language before/alongside your Rust code via Ant, Groovy, etc.
* **TODO?:** Auto-locate tools based on env vars, common paths, etc.
* **TODO?:** Auto-install missing tools for you.

## Non-Goals

* Directly compete with Ant, Groovy, etc. as a fully fledged Java build tool.

## Java <-> Rust Interop Pontificating

Rust code may sanely depend on Java code to build, but not vicea versa:
* Java's ABI has great metadata (classes, methods, doc info, etc.), Rust doesn't even have a stable ABI.
* Local Java <- Rust <- Java dependency cycles at compile time would be terrible to manage.

You can still have:
* Java call into Rust, it should just be through `native` methods declared in Java.
* Rust-driven packaging bundle Java JARs (e.g. when creating Android APKs) as a final step.
* Rust define runtime classes implementing interfaces, they just wouldn't be visible to Java at compile time.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

<!-- https://doc.rust-lang.org/1.4.0/complement-project-faq.html#why-dual-mit/asl2-license? -->
<!-- https://rust-lang-nursery.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive -->
<!-- https://choosealicense.com/licenses/apache-2.0/ -->
<!-- https://choosealicense.com/licenses/mit/ -->

[cc]:                       https://crates.io/crates/cc
[`kotlinc`]:                https://kotlinlang.org/docs/tutorials/command-line.html
[build.rs]:                 https://doc.rust-lang.org/cargo/reference/build-scripts.html
[metabuild]:                https://github.com/rust-lang/rfcs/blob/master/text/2196-metabuild.md
