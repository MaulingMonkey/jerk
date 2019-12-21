use std::path::PathBuf;
use std::env::var_os;

fn main() {
    jerk_build::metabuild();
    println!("cargo:rustc-link-search=native={}", PathBuf::from(var_os("JAVA_HOME").expect("${JAVA_HOME} not set")).join("lib").display());
}
