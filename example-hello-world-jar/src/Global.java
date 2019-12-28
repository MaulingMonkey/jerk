package com.maulingmonkey.jerk.example_hello_world_jar;

public class Global {
    static native void assert_native_value(int value);

    static void assert_value_eq(int value) {
        // https://github.com/MaulingMonkey/jerk/issues/14
        System.load(System.getProperty("com.maulingmonkey.jerk_test.jni_symbols_source"));
        assert_native_value(value);
    }

    static void assert_value_eq_1() { assert_value_eq(1); }
    static void assert_value_eq_3() { assert_value_eq(3); }
    static void assert_value_eq_5() { assert_value_eq(5); }
}
