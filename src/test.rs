use crate::*;
use crate::jvm::JniError;
use jni_sys::*;
use std::convert::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::ptr::null_mut;
use std::sync::Mutex;

pub type Result<T> = std::result::Result<T, JavaTestError>;

#[derive(Clone)]
pub enum JavaTestError {
    Unknown(String),
    #[doc(hidden)] _NonExhaustive,
}

impl Display for JavaTestError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            JavaTestError::Unknown(message) => write!(fmt, "{}", message),
            JavaTestError::_NonExhaustive   => write!(fmt, "NonExhaustive"),
        }
    }
}

impl Debug for JavaTestError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        Display::fmt(self, fmt)
    }
}

impl<'a> From<&'a str> for JavaTestError {
    fn from(value: &'a str) -> Self {
        JavaTestError::Unknown(value.to_string())
    }
}


impl From<String> for JavaTestError {
    fn from(value: String) -> Self {
        JavaTestError::Unknown(value)
    }
}


/// Execute a Java unit test.  The method must be static, return void, and take no arguments.
#[macro_export] macro_rules! run_test {
    ( $package:expr, $class:expr, $method:expr ) => {{
        $crate::test::run_test_impl(env!("JERK_BUILD_JAR"), $package, $class, $method).unwrap()
    }};
}

#[doc(hidden)]
pub fn run_test_impl(jar: &str, package: &str, class: &str, method: &str) -> Result<()> {
    let mut reused_vm = false;
    {
        let mut vm = VM.lock().unwrap();
        if vm.is_null() {
            **vm = create_java_vm(jar, &mut reused_vm);
        }
    }

    let env = test_thread_env();
    if env == null_mut() { return Err("Couldn't initialize Java VM".into()); }

    let class_id    = format!("{}/{}\0", package.replace(".", "/"), class);
    let method_id   = format!("{}\0", method);

    // Safety:
    // * `**env` must be valid (non-null, not dangling, valid fn pointers if present)
    // * string IDs must be `\0` terminated
    unsafe {
        let class_id = (**env).FindClass.unwrap()(env, class_id.as_ptr() as *const _);
        if class_id.is_null() {
            panic!(
                concat!(
                    "Failed to find class {package}.{class}.  Possible causes:\n",
                    "{reused_vm_warning}",
                    "  - Typos in the package or class name.\n",
                    "  - The corresponding .jar may have been built with a newer JDK (are you mixing old and new JDKs for 32-bit and 64-bit?)\n",
                    "  - The corresponding .jar may not be have been found (are you using `jerk_build::metabuild()` in your build.rs?)\n",
                ),
                package=package, class=class,
                reused_vm_warning = if reused_vm { "  - WARNING: jerk::run_test! was unable to start a new JVM / specify a java.class.path.  The .jar may not have been loaded!\n" } else { "" },
            );
        }
        let method_id   = (**env).GetStaticMethodID.unwrap()(env, class_id, method_id.as_ptr() as *const _, "()V\0".as_ptr() as *const _);
        assert_ne!(method_id, null_mut(), "Failed to GetStaticMethodID {}.{}", class, method);
        (**env).CallStaticVoidMethodA.unwrap()(env, class_id, method_id, [].as_ptr());
        if (**env).ExceptionCheck.unwrap()(env) == JNI_TRUE {
            (**env).ExceptionDescribe.unwrap()(env);
            (**env).ExceptionClear.unwrap()(env);
            Err(format!("{}.{}() threw a Java Exception", class, method).into())
        } else {
            Ok(())
        }
    }
}


lazy_static::lazy_static! { static ref JVM : jvm::Library = jvm::Library::get().unwrap(); }

/// Get a handle to the current Java VM, or create one if it doesn't already exist.
pub fn test_vm() -> *mut JavaVM {
    let vm = **VM.lock().unwrap();
    debug_assert!(!vm.is_null(), "VM is null, are you trying to access the test_vm outside of a `run_test!`?");
    vm
}
lazy_static::lazy_static! { static ref VM : Mutex<ThreadSafe<*mut JavaVM>> = Mutex::new(ThreadSafe(null_mut())); }

/// Get a handle to the Java environment for the current thread, attaching if one doesn't already exist.
pub fn test_thread_env() -> *mut JNIEnv { ENV.with(|e| *e) }
thread_local! { static ENV : *mut JNIEnv = attach_current_thread(); }

fn attach_current_thread() -> *mut JNIEnv {
    let vm = test_vm();
    let mut env = null_mut();
    assert_eq!(JNI_OK, unsafe { (**vm).AttachCurrentThread.unwrap()(vm, &mut env, null_mut()) });
    env as *mut _
}

fn create_java_vm(jar: &str, reused_vm: &mut bool) -> *mut JavaVM {
    match JVM.create_java_vm(vec![
        //"-verbose:class".to_string(),
        //"-verbose:jni".to_string(),
        "-ea".to_string(),  // Enable Assertions
        "-esa".to_string(), // Enable System Assertions
        format!("-Djava.class.path={}", jar),
    ]) {
        Err(JniError::EXIST) => {
            if let Some(vm) = JVM.get_created_java_vms().unwrap().into_iter().next() {
                *reused_vm = true;
                vm
            } else {
                panic!("JNI_EEXIST error creating Java VM, but unable to get an existing VM");
            }
        },
        Err(err) => panic!("JNI error creating Java VM: {}", err),
        Ok(vm) => vm
    }

}

struct ThreadSafe<T>(pub T);
impl<T> std::ops::Deref for ThreadSafe<T> { type Target = T; fn deref(&self) -> &Self::Target { &self.0 } }
impl<T> std::ops::DerefMut for ThreadSafe<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }
unsafe impl<T> Send for ThreadSafe<T> {}
unsafe impl<T> Sync for ThreadSafe<T> {}
