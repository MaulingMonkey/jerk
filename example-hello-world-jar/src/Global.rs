use jni_sys::{JNIEnv, jobject, jint};
use std::sync::atomic::{AtomicI32, Ordering};

static VALUE : AtomicI32 = AtomicI32::new(0);

#[no_mangle] pub extern "stdcall" fn Java_com_maulingmonkey_jerk_example_1hello_1world_1jar_Global_assert_1native_1value__I(_env: *mut JNIEnv, _this: jobject, expected_value: jint) {
    let value = VALUE.load(Ordering::SeqCst);
    if value != expected_value {
        // Don't unwind across FFI boundaries when we fail this test
        eprintln!("assert_native_value:  {} != {}", value, expected_value);
        std::process::exit(1);
    }
}


#[no_mangle] pub extern "stdcall" fn Java_com_maulingmonkey_jerk_example_1hello_1world_1jar_Global_test() {
    // https://github.com/MaulingMonkey/jerk/issues/12
    // 
    // Loading a separately build cdylib actually caused a whole second copy of
    // the entire crate to be loaded with it's own separate static vars and
    // everything.  This test would catch that.
    VALUE.store(1, Ordering::SeqCst); jerk_test::run_test("com.maulingmonkey.jerk.example_hello_world_jar", "Global", "assert_value_eq_1").unwrap();
    VALUE.store(3, Ordering::SeqCst); jerk_test::run_test("com.maulingmonkey.jerk.example_hello_world_jar", "Global", "assert_value_eq_3").unwrap();
    VALUE.store(5, Ordering::SeqCst); jerk_test::run_test("com.maulingmonkey.jerk.example_hello_world_jar", "Global", "assert_value_eq_5").unwrap();
}
