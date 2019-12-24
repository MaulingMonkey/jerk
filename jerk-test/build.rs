fn main() {
    println!("cargo:rustc-env=PROFILE={}", std::env::var("PROFILE").expect("PROFILE not set or invalid unicode"));
}
