use std::path::PathBuf;
use std::env::var_os;

fn main() {
    jerk_build::metabuild();
    let java_home = PathBuf::from(var_os("JAVA_HOME").expect("${JAVA_HOME} not set"));
    println!("cargo:rustc-link-search=native={}", java_home.join("lib").display());
    println!("cargo:rustc-link-search=native={}", java_home.join("jre/lib/amd64/server").display());
}
