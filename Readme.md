# **jerk**: **J**ava **E**mbedding **R**ust **K**it

[![Crates.io](https://img.shields.io/crates/v/jerk.svg)](https://crates.io/crates/jerk)
[![Docs](https://docs.rs/jerk/badge.svg)](https://docs.rs/jerk/)
[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/jerk.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/jerk)
[![unsafe: yes](https://img.shields.io/github/search/MaulingMonkey/jerk/unsafe%2bextension%3Ars?color=yellow&label=unsafe)](https://github.com/MaulingMonkey/jerk/search?q=unsafe+extension%3Ars)
[![rust: 1.54.0+](https://img.shields.io/badge/rust-1.54.0%2B-green.svg)](https://gist.github.com/MaulingMonkey/c81a9f18811079f19326dac4daa5a359#minimum-supported-rust-versions-msrv)
[![License](https://img.shields.io/crates/l/jerk.svg)](https://github.com/MaulingMonkey/jerk)

Libraries to compile/embed/test Java alongside a Rust library/application.
Similar to [cc], but for Java.
This is **not** an official project of Google, Oracle, Sun Microsystems, or anyone else.


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

## Quick Start

[Install the JDK](https://github.com/MaulingMonkey/jerk/wiki/Installing-the-JDK) if you haven't already.

Add this to your [Cargo.toml](https://github.com/MaulingMonkey/jerk/blob/master/example-hello-world-jar/Cargo.toml#L10-L20):

```toml
[lib]
crate-type = ["rlib", "dylib"]

[dependencies]
jni-sys     = "0.3"

[build-dependencies]
jerk        = "0.2"

[dev-dependencies]
jerk        = "0.2"
```

And this to your [build.rs](https://github.com/MaulingMonkey/jerk/blob/master/example-hello-world-jar/build.rs):

```rust
fn main() {
    jerk::metabuild();
}
```

You can then write Java ([src/Adder.java](https://github.com/MaulingMonkey/jerk/blob/master/example-hello-world-jar/src/Adder.java)) code:

```java
package com.maulingmonkey.jerk.example_hello_world_jar;
public class Adder {
    public native int add(int a, int b);
    public static void test() {
        System.loadLibrary("example_hello_world_jar");
        assert adder.add(1, 2) == 3;
    }
}
```

...alongside your Rust ([src/Adder.rs](https://github.com/MaulingMonkey/jerk/blob/master/example-hello-world-jar/src/Adder.rs)) code:

```rust
use jni_sys::{JNIEnv, jobject, jint};
#[no_mangle] pub extern "stdcall" fn Java_com_maulingmonkey_jerk_example_1hello_1world_1jar_Adder_add__II(_env: *mut JNIEnv, _this: jobject, a: jint, b: jint) -> jint {
    a + b
}
```

...and write Java integration tests ([tests/test.rs](https://github.com/MaulingMonkey/jerk/blob/master/example-hello-world-jar/tests/test.rs)):

```rust
#[test] fn test() {
    jerk::run_test!("com.maulingmonkey.jerk.example_hello_world_jar", "Adder", "test");
}
```

...and then build and run the test!

```text
C:\local\jerk>cargo t
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running target\debug\deps\example_hello_world_jar-2997df28c387b743.exe

running 1 tests
test adder::test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

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

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

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
