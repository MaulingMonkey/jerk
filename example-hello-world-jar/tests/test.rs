extern crate example_hello_world_jar; // Force it to be built/used

#[test] fn test() {
    jerk::run_test!("com.maulingmonkey.jerk.example_hello_world_jar", "Adder",  "test");
    jerk::run_test!("com.maulingmonkey.jerk.example_hello_world_jar", "Global", "test");
}
