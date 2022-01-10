#[test] fn test() {
    jerk::run_test!("com.maulingmonkey.jerk.example_hello_world_jar", "Adder",  "test");
    jerk::run_test!("com.maulingmonkey.jerk.example_hello_world_jar", "Global", "test");
}
