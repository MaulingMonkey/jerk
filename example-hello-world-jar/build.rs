fn main() {
    jerk_build::metabuild();
    // https://github.com/MaulingMonkey/jerk/issues/15
    println!("cargo:rustc-env=LINK=/DEF:{}", std::path::Path::new("exports.def").canonicalize().unwrap().display());
}
