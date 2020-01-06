#[test] fn test() {
    example_hello_world_jar::force_me_to_build();
    jerk_test::run_test("com.maulingmonkey.jerk.example_hello_world_jar", "Adder",  "test").unwrap();
    jerk_test::run_test("com.maulingmonkey.jerk.example_hello_world_jar", "Global", "test").unwrap();
}
