extern crate example_hello_world_jar; // Force it to be built/used

#[test] fn test() {
    jerk_test::run_test!("com.maulingmonkey.jerk.example_hello_world_jar", "Adder",  "test");
    jerk_test::run_test!("com.maulingmonkey.jerk.example_hello_world_jar", "Global", "test");
}
