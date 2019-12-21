use crate::*;
use std::convert::{AsRef};
use std::fs;

/// A cargo [metabuild] compatible entry point.
/// 
/// # To consume via [build.rs]
/// 
/// Add the following to your executable's Cargo.toml:
/// 
/// ```toml
/// [build-dependencies]
/// jerk = "0.1"
/// ```
/// 
/// And the following to your [build.rs]:
/// ```no_run
/// fn main() {
///     jerk_build::metabuild();
/// }
/// ```
/// 
/// # To consume via [metabuild] (nightly only)
/// 
/// Add the following to your executable's Cargo.toml:
/// 
/// ```toml
/// cargo-features = ["metabuild"]
/// 
/// [package]
/// metabuild = ["jerk"]
/// 
/// [build-dependencies]
/// jerk = "0.1"
/// ```
/// 
/// [build.rs]:             https://doc.rust-lang.org/cargo/reference/build-scripts.html
/// [metabuild]:            https://github.com/rust-lang/rfcs/blob/master/text/2196-metabuild.md
pub fn metabuild() {
    let java_home   = search::find_java_home().expect("${JAVA_HOME} is not set and cannot be inferred");
    println!("rustc-env=JAVA_HOME={}", java_home.display());
    env::set_var("JAVA_HOME", &java_home);

    println!("cargo:rustc-link-lib=static=jvm");
    println!("cargo:rustc-link-search=native={}", java_home.display());

    //let cwd           = env::current_dir().unwrap();
    let metadata        = cargo_metadata::MetadataCommand::new().exec().expect("cargo-metadata failed");
    let profile         = env::var("PROFILE").expect("${PROFILE} is not set or is invalid Unicode");
    let package_name    = env::var("CARGO_PKG_NAME").expect("${CARGO_PKG_NAME} is not set or is invalid Unicode");
    //let out_dir       = env::var_path("OUT_DIR").expect("${OUT_DIR} is not set or is invalid Unicode");

    let debug_info = match profile.as_str() {
        "debug"     => Some(javac::DebugInfo::ALL),
        "release"   => Some(javac::DebugInfo::NONE), // XXX: Check if rust is building w/ symbols instead?
        _custom     => None,
    };

    let java_home = Some(java_home.as_ref());
    let out_java = metadata.target_directory.join(profile).join("java");
    let out_classes = out_java.join("classes");
    let out_sources = out_java.join("source" );
    let out_headers = out_java.join("headers");
    let out_jars    = out_java.join("jars");
    let out_jar     = out_jars.join(format!("{}.jar", package_name));
    let _ = fs::create_dir_all(&out_java);
    let _ = fs::create_dir(&out_classes);
    let _ = fs::create_dir(&out_sources);
    let _ = fs::create_dir(&out_headers);
    let _ = fs::create_dir(&out_jars);

    javac::Compile {
        java_home,
        debug_info,
        out_classes: Some(out_classes.as_ref()),
        out_sources: Some(out_sources.as_ref()),
        out_headers: Some(out_headers.as_ref()),
        files: &["src/*.java".as_ref()][..],
        ..javac::Compile::default()
    }.exec().unwrap();

    jar::Archive {
        java_home,
        jar_file:   Some(out_jar.as_ref()),
        files:&[
            (out_classes.as_ref(), &[".".as_ref()][..]),
        ][..],
        ..jar::Archive::default()
    }.create().unwrap();
}
