#![cfg_attr(feature = "nightly", feature(doc_cfg)       )] // https://doc.rust-lang.org/unstable-book/language-features/doc-cfg.html
#![cfg_attr(feature = "nightly", feature(external_doc)  )] // https://doc.rust-lang.org/unstable-book/language-features/external-doc.html
#![cfg_attr(feature = "nightly", doc(include = "../Readme.md"))]

mod arch;
mod env;
#[allow(dead_code)] mod jar;    // TODO: Make public after finalizing APIs?
#[allow(dead_code)] mod java;   // TODO: Make public after finalizing APIs?
#[allow(dead_code)] mod javac;  // TODO: Make public after finalizing APIs?
pub mod jvm;
mod metabuild;
pub mod paths;
mod search;
#[macro_use] pub mod test;

pub use arch::Arch;
pub use metabuild::metabuild;
