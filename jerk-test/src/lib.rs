#![cfg_attr(feature = "nightly", feature(external_doc)  )] // https://doc.rust-lang.org/unstable-book/language-features/external-doc.html
#![cfg_attr(feature = "nightly", doc(include = "../Readme.md"))]

use jni_sys::*;
use std::convert::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::path::{Component, Path, PathBuf};
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
        assert_ne!(
            class_id,
            null_mut(),
            concat!(
                "Failed to FindClass {}.{}.  Possible causes:\n",
                "  - Typos in the package or class name.\n",
                "  - The corresponding .jar may have been built with a newer JDK (are you mixing old and new JDKs for 32-bit and 64-bit?)\n",
                "  - The corresponding .jar may not be have been found (are you using `jerk_build::metabuild()` in your build.rs?)\n",
            ),
            package,
            class
        );
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
    // https://github.com/MaulingMonkey/jerk/issues/14
    let jni_symbol_source = PathBuf::from(std::env::args_os()
        .next().expect("Unable to determine test EXE"))
        .canonicalize().expect("Unable to determine absolute path of test EXE") // System.load() requires an absolute path
        .components().map(fix_drive_prefix).collect::<PathBuf>();               // System.load() cannot handle a "\\?\" prefix

    JVM.create_java_vm(vec![
        //"-verbose:class".to_string(),
        //"-verbose:jni".to_string(),
        "-ea".to_string(),  // Enable Assertions
        "-esa".to_string(), // Enable System Assertions
        format!("-Djava.class.path={}", find_jar().display()),
        format!("-Dcom.maulingmonkey.jerk_test.jni_symbols_source={}", jni_symbol_source.display()),
    ]).unwrap()

}

fn find_jar() -> PathBuf {
    // We're assuming here that *our* profile is the same as the *test* profile.
    // That's technically a bad assumption and likely to change in the future,
    // if/when cargo gains support for mixing and matching different build
    // profiles for different crates.  Or it'll even break *right now* if you
    // manually specify rlibs yourself like some kind of madman.
    // 
    // It's also possible to override the output directory placing this jar
    // elsewhere, but that's not an easily solved problem anyways.  OUT_DIR only
    // gets set if you have a build.rs, which isn't guaranteed... although if
    // the .jar is in this specific location, you probably have one that
    // runs jerk_build::metabuild().
    let relative = PathBuf::from(format!("target/{profile}/java/jars/{pkg_name}.jar", profile=env!("PROFILE"), pkg_name=std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME not set or invalid unicode")));

    // Okay, go actually find that jar.
    let mut dir = std::env::current_dir().expect("Couldn't get current directory");
    while !dir.join(&relative).exists() {
        assert!(
            dir.pop(),
            concat!(
                "Cannot find {jar}.  Possible causes:\n",
                "  - You're not using jerk_build::metabuild() in your build.rs.\n",
                "  - You've overridden the target/output directory.\n",
                "  - You're running tests from a weird directory.\n",
            ),
            jar = relative.display()
        );
    }
    dir.join(relative)
}

fn fix_drive_prefix(c: Component) -> Component {
    match c {
        Component::Prefix(p) => {
            if let Some(s) = p.as_os_str().to_str() {
                if s.starts_with(r"\\?\") {
                    Path::new(&s[4..]).components().next().unwrap_or(c)
                } else {
                    c
                }
            } else {
                c
            }
        },
        other => other,
    }
}

struct ThreadSafe<T>(pub T);
impl<T> std::ops::Deref for ThreadSafe<T> { type Target = T; fn deref(&self) -> &Self::Target { &self.0 } }
unsafe impl<T> Send for ThreadSafe<T> {}
unsafe impl<T> Sync for ThreadSafe<T> {}
