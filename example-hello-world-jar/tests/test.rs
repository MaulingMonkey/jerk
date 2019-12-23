
#[test] fn test() -> Result<(), jerk_test::JavaTestError> {
    jerk_test::run_test("com.maulingmonkey.jerk.example_hello_world_jar", "Adder", "test")
}
