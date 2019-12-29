#[test] fn test() -> Result<(), jerk_test::JavaTestError> {
    example_hello_world_jar::force_link();
    jerk_test::run_test("com.maulingmonkey.jerk.example_hello_world_jar", "Adder", "test")
}
