use jerk_test::*;

const PACKAGE : &'static str = "com.maulingmonkey.jerk.example_hello_world_jar";

#[test] fn adder_test() -> Result<()> { run_test(PACKAGE, "Adder", "test") }
