#![cfg_attr(feature = "nightly", feature(external_doc)  )] // https://doc.rust-lang.org/unstable-book/language-features/external-doc.html
#![cfg_attr(feature = "nightly", doc(include = "../Readme.md"))]

use jni_sys::*;
use std::convert::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use std::ptr::null_mut;

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
pub fn run_test(package: &str, class: &str, method: &str) -> Result<()> {
    let env = test_thread_env();
    if env == null_mut() { return Err("Couldn't initialize Java VM".into()); }
    
    let class_id    = format!("{}/{}\0", package.replace(".", "/"), class);
    let method_id   = format!("{}\0", method);
    
    // Safety:
    // * `**env` must be valid (non-null, not dangling, valid fn pointers if present)
    // * string IDs must be `\0` terminated
    unsafe {
        let class_id    = (**env).FindClass.unwrap()(env, class_id.as_ptr() as *const _);
        assert_ne!(class_id, null_mut(), "Failed to FindClass {}.{} - the corresponding .jar may not be loaded", package, class);
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


lazy_static::lazy_static! { static ref JVM : jerk::jvm::Library = jerk::jvm::Library::get().unwrap(); }

/// Get a handle to the current Java VM, or create one if it doesn't already exist.
pub fn test_vm() -> *mut JavaVM { **VM }
lazy_static::lazy_static! { static ref VM : ThreadSafe<*mut JavaVM> = ThreadSafe(create_java_vm()); }

/// Get a handle to the Java environment for the current thread, attaching if one doesn't already exist.
pub fn test_thread_env() -> *mut JNIEnv { ENV.with(|e| *e) }
thread_local! { static ENV : *mut JNIEnv = attach_current_thread(); }

fn attach_current_thread() -> *mut JNIEnv {
    let vm = test_vm();
    let mut env = null_mut();
    assert_eq!(JNI_OK, unsafe { (**vm).AttachCurrentThread.unwrap()(vm, &mut env, null_mut()) });
    env as *mut _
}

fn create_java_vm() -> *mut JavaVM {
    JVM.create_java_vm(vec![
        //"-verbose:class".to_string(),
        //"-verbose:jni".to_string(),
        "-ea".to_string(),  // Enable Assertions
        "-esa".to_string(), // Enable System Assertions
        format!("-Djava.class.path={profile_dir}/java/jars/{pkg_name}.jar", profile_dir=taget_profile_dir().display(), pkg_name=std::env::var("CARGO_PKG_NAME").unwrap()),
    ]).unwrap()
}

fn taget_profile_dir() -> PathBuf {
    let mut dir = jerk::paths::env("OUT_DIR").unwrap();
    while !is_profile_dir(&dir) && dir.pop() {}
    eprintln!("{}", dir.display());
    dir
}

fn is_profile_dir(path: &Path) -> bool {
    if path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()) != Some("target") { return false; }
    if !path.join("java").exists() { return false; }
    true
}

struct ThreadSafe<T>(pub T);
impl<T> std::ops::Deref for ThreadSafe<T> { type Target = T; fn deref(&self) -> &Self::Target { &self.0 } }
unsafe impl<T> Send for ThreadSafe<T> {}
unsafe impl<T> Sync for ThreadSafe<T> {}
