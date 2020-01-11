use crate::*;
use std::convert::{AsRef};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// A cargo [metabuild] compatible entry point.
/// 
/// # To consume via [build.rs]
/// 
/// Add the following to your executable's Cargo.toml:
/// 
/// ```toml
/// [build-dependencies]
/// jerk = "0.2"
/// ```
/// 
/// And the following to your [build.rs]:
/// ```no_run
/// fn main() {
///     jerk::metabuild();
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
/// jerk = "0.2"
/// ```
/// 
/// [build.rs]:             https://doc.rust-lang.org/cargo/reference/build-scripts.html
/// [metabuild]:            https://github.com/rust-lang/rfcs/blob/master/text/2196-metabuild.md
pub fn metabuild() {
    let java_home = paths::java_home().unwrap();
    println!("cargo:rerun-if-env-changed=JAVA_HOME");
    println!("cargo:rustc-env=JAVA_HOME={}", java_home.display());
    env::set_var("JAVA_HOME", &java_home);

    let profile         = env::var("PROFILE").expect("${PROFILE} is not set or is invalid Unicode");
    let package_name    = env::var("CARGO_PKG_NAME").expect("${CARGO_PKG_NAME} is not set or is invalid Unicode");
    let out_dir         = env::var_path("OUT_DIR").expect("${OUT_DIR} is not set or is invalid Unicode");

    let debug_info = match profile.as_str() {
        "debug"     => Some(javac::DebugInfo::ALL),
        "release"   => Some(javac::DebugInfo::NONE), // XXX: Check if rust is building w/ symbols instead?
        _custom     => None,
    };

    let mut files = Vec::new();
    find_java_srcs(Path::new("."), &mut files).unwrap_or_else(|err| panic!("Failed to enumerate/read Java source code: {}", err));
    // This is a very limited and kinda poor heuristic for detecting entry points.  https://github.com/MaulingMonkey/jerk/issues/18
    let entry_point = if files.iter().find(|p| p.file_name() == Some(std::ffi::OsStr::new("Main.java"))).is_some() { Some(String::from("Main")) } else { None };

    let out_java    = out_dir.join("java");
    let out_classes = out_java.join("classes");
    let out_sources = out_java.join("source" );
    let out_headers = out_java.join("headers");
    let out_jars    = out_java.join("jars");

    let mut path;
    let out_jar = if entry_point.is_none() {
        &out_jars
    } else {
        path = out_dir.clone();
        let ok =
            path.file_name().and_then(|s| s.to_str()).map_or(false, |n| n == "out")                     && path.pop() &&
            path.file_name().and_then(|s| s.to_str()).map_or(false, |n| n.starts_with(&package_name))   && path.pop() &&
            path.file_name().and_then(|s| s.to_str()).map_or(false, |n| n == "build")                   && path.pop();

        if ok {
            &path
        } else {
            println!("cargo:warning=Expected OUT_DIR {:?} to end with build/{}-.../out", out_dir, package_name);
            &out_jars
        }
    }.join(format!("{}.jar", package_name));

    let _ = fs::create_dir_all(&out_java);
    let _ = fs::create_dir(&out_classes);
    let _ = fs::create_dir(&out_sources);
    let _ = fs::create_dir(&out_headers);
    let _ = fs::create_dir(&out_jars);

    javac::Compile {
        java_home: Some(java_home.clone()),
        debug_info,
        out_classes: Some(out_classes.clone()),
        out_sources: Some(out_sources),
        out_headers: Some(out_headers),
        files,
        ..javac::Compile::default()
    }.exec().unwrap();

    jar::Archive {
        entry_point,
        java_home:      Some(java_home.as_ref()),
        jar_file:       Some(out_jar.as_ref()),
        files:          &[(out_classes.as_ref(), &[".".as_ref()][..])][..],
        ..jar::Archive::default()
    }.create().unwrap();

    println!("cargo:rustc-env=JERK_BUILD_JAR={}", out_jar.display());
}

fn find_java_srcs(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    println!("cargo:rerun-if-changed={}", path.display()); // rerun if directory might add/remove source files
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name_lossy = name.to_string_lossy();
        const DOT_JAVA : &'static str = ".java";

        if path.is_dir() {
            find_java_srcs(&path, files)?;
        } else if name_lossy.get(name_lossy.len()-DOT_JAVA.len()..).map(|ext| ext.eq_ignore_ascii_case(DOT_JAVA)).unwrap_or(false) {
            println!("cargo:rerun-if-changed={}", path.display()); // rerun if source file changed
            files.push(path);
        }
    }
    Ok(())
}
