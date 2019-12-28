#![rustversion::attr(nightly, feature(doc_cfg)       )] // https://doc.rust-lang.org/unstable-book/language-features/doc-cfg.html
#![rustversion::attr(nightly, feature(external_doc)  )] // https://doc.rust-lang.org/unstable-book/language-features/external-doc.html
#![rustversion::attr(nightly, doc(include = "../Readme.md"))]

#[macro_use] extern crate dlopen_derive;

pub mod jvm;
pub mod paths;
