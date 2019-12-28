package com.maulingmonkey.jerk.example_hello_world_jar;

public class Adder {
    public native String add(String a, String b);
    public native float add(float a, float b);
    public native int add(int a, int b);

    public static void test() {
        // https://github.com/MaulingMonkey/jerk/issues/14
        System.load(System.getProperty("com.maulingmonkey.jerk_test.jni_symbols_source"));
        Adder adder = new Adder();
        assert adder.add("1", "2").equals("12");
        assert adder.add(1.0f, 2.0f) == 3.0f;
        assert adder.add(1, 2) == 3;
    }
}
