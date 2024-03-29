#![cfg_attr(feature = "nightly", feature(doc_cfg))] // https://doc.rust-lang.org/unstable-book/language-features/doc-cfg.html
#![doc = include_str!("../Readme.md")]

mod env;
#[allow(dead_code)] mod jar;    // TODO: Make public after finalizing APIs?
#[allow(dead_code)] mod java;   // TODO: Make public after finalizing APIs?
#[allow(dead_code)] mod javac;  // TODO: Make public after finalizing APIs?
pub mod jvm;
mod metabuild;
pub mod paths;
mod search;
#[macro_use] pub mod test;

pub use metabuild::metabuild;
