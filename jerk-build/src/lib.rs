#![rustversion::attr(nightly, feature(external_doc)  )] // https://doc.rust-lang.org/unstable-book/language-features/external-doc.html
#![rustversion::attr(nightly, doc(include = "../Readme.md"))]

mod env;
#[allow(dead_code)] mod jar;    // TODO: Make public after finalizing APIs?
#[allow(dead_code)] mod java;   // TODO: Make public after finalizing APIs?
#[allow(dead_code)] mod javac;  // TODO: Make public after finalizing APIs?
mod metabuild;
mod search;

pub use metabuild::metabuild;
