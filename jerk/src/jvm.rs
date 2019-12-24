//! Java VM Management APIs
#![allow(non_snake_case)] // JNI nonsense

use crate::paths;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::os::raw::*;
use std::path::{Path};
use std::ptr::{null_mut};
use dlopen::wrapper::{Container, WrapperApi};
use jni_sys::*;
#[cfg(windows)] use winapi::shared::winerror::*;

/// Error loading a [Library]
/// 
/// [Library]:      struct.Library.html
#[derive(Debug)]
pub struct LoadError(dlopen::Error);
impl Display                for LoadError { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.0, fmt) } }
impl Error                  for LoadError { fn source(&self) -> Option<&(dyn Error + 'static)> { self.0.source() } }
impl From<dlopen::Error>    for LoadError { fn from(error: dlopen::Error) -> Self { Self(error) } }
impl From<io::Error>        for LoadError { fn from(error: io::Error) -> Self { Self(dlopen::Error::OpeningLibraryError(error)) } }

/// Error calling a [Library] function
/// 
/// [Library]:      struct.Library.html
#[derive(Debug)]
pub struct JniError(jint);
impl Display for JniError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.0 {
            jni_sys::JNI_OK         => write!(fmt, "JNI_OK"),
            jni_sys::JNI_EDETACHED  => write!(fmt, "JNI_EDETACHED"),
            jni_sys::JNI_EVERSION   => write!(fmt, "JNI_EVERSION"),
            jni_sys::JNI_ENOMEM     => write!(fmt, "JNI_ENOMEM"),
            jni_sys::JNI_EEXIST     => write!(fmt, "JNI_EEXIST"),
            jni_sys::JNI_EINVAL     => write!(fmt, "JNI_EINVAL"),
            unknown                 => write!(fmt, "JNI_??? ({})", unknown),
        }
    }
}
impl Error   for JniError {}

#[derive(WrapperApi)]
struct JVMAPI {
    JNI_CreateJavaVM:               unsafe extern "system" fn (pvm: *mut *mut JavaVM, penv: *mut *mut c_void, args: *mut c_void) -> jint,
    JNI_GetCreatedJavaVMs:          unsafe extern "system" fn (vmBuf: *mut *mut JavaVM, bufLen: jsize, nVMs: *mut jsize) -> jint,
    JNI_GetDefaultJavaVMInitArgs:   unsafe extern "system" fn (args: *mut c_void) -> jint,
}

/// Represents a loaded `jvm.dll` or `libjvm.so` instance.
pub struct Library {
    jvm: Container<JVMAPI>,
}

impl Library {
    /// Get an instance of the library by... whatever logic `jerk` feels like.  This currently means searching in this order:
    /// 
    /// * Already loaded symbols, in case Java is hosting us (`.jar` entry point like on Android) instead of us hosting Java
    /// * `%JAVA_HOME%`, if set
    /// * Various JDK locations that could totally have been set as `%JAVA_HOME%`
    pub fn get() -> Result<Library, LoadError> {
        Self::from_already_loaded()
            .or_else(|_| Self::from_system())
    }

    /// Reference the already loaded JVM library.
    pub fn from_already_loaded() -> Result<Library, LoadError> {
        let jvm = unsafe { Container::load_self() }?;
        Ok(Self{jvm})
    }

    /// Load a JVM library from wherever.
    #[cfg_attr(feature = "nightly", doc(cfg(not(target_os = "android"))))] // We actually still compile this in but discourage it as unlikely to work...
    pub fn from_system() -> Result<Library, LoadError> {
        let java_home = paths::java_home()?;
        Self::from_java_home(&java_home)
    }

    /// Load a JVM library from a specific `%JAVA_HOME%`.
    /// 
    /// # Arguments
    /// 
    /// - `java_home` - this should be `%JAVA_HOME%` or similar.
    #[cfg_attr(feature = "nightly", doc(cfg(not(target_os = "android"))))] // We actually still compile this in but discourage it as unlikely to work...
    pub fn from_java_home(java_home: &(impl AsRef<Path> + ?Sized)) -> Result<Library, LoadError> {
        let java_home = java_home.as_ref();
        let libjvm_dir = paths::libjvm_dir(&java_home)?;
        Self::from_library_path(&libjvm_dir.join(if cfg!(windows) { "jvm.dll" } else { "libjvm.so" }))
    }

    /// Load a JVM library from a specific path.
    /// 
    /// # Arguments
    /// 
    /// - `libjvm` - this should be a path to `jvm.dll` / `libjvm.so`
    #[cfg_attr(feature = "nightly", doc(cfg(not(target_os = "android"))))] // We actually still compile this in but discourage it as unlikely to work...
    pub fn from_library_path(libjvm: &(impl AsRef<Path> + ?Sized)) -> Result<Library, LoadError> {
        let libjvm = libjvm.as_ref();
        let jvm = unsafe { Container::load(libjvm) }
            .map_err(|err| match err {
                // "%1 is not a valid Win32 application." - likely caused by architecture mismatch
                #[cfg(windows)] dlopen::Error::OpeningLibraryError(io) if io.kind() == io::ErrorKind::Other && io.raw_os_error() == Some(ERROR_BAD_EXE_FORMAT as _) => {
                    dlopen::Error::OpeningLibraryError(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            concat!(
                                "Unable to load {}: ERROR_BAD_EXE_FORMAT\r\n",
                                "This is likely caused by trying to use 32-bit Java from a 64-bit Rust binary or vicea versa.\r\n",
                                "This in turn is likely caused by not having a corresponding Java installation.\r\n"
                            ),
                            libjvm.display(),
                        )
                    ))
                },
                other => other,
            })?;
        Ok(Self{jvm})
    }

    /// `JNI_CreateJavaVM`
    #[cfg_attr(feature = "nightly", doc(cfg(not(target_os = "android"))))] // We actually still compile this in but discourage it as unlikely to work...
    pub fn create_java_vm(&self, mut java_vm_options: Vec<String>) -> Result<*mut JavaVM, JniError> {
        for o in java_vm_options.iter_mut() {
            // XXX: Assert doesn't contain nul?  Escape to weird Java psuedo-UTF8 "nuls"?
            o.push('\0');
        }

        let mut java_vm_options : Vec<JavaVMOption> = java_vm_options.iter_mut().map(|o| JavaVMOption {
            optionString:   o.as_mut_ptr() as *mut _,
            extraInfo:      null_mut(),
        }).collect();

        let mut args = JavaVMInitArgs {
            version:            JNI_VERSION_1_6,
            nOptions:           java_vm_options.len() as _,
            options:            java_vm_options.as_mut_ptr(),
            ignoreUnrecognized: JNI_FALSE,
        };

        let mut vm = null_mut();
        let mut env = null_mut();
        let r = unsafe { self.jvm.JNI_CreateJavaVM(&mut vm, &mut env, &mut args as *mut _ as *mut _) };
        if r == JNI_OK {
            Ok(vm)
        } else {
            Err(JniError(r))
        }
    }

    /// `JNI_GetCreatedJavaVMs`
    pub fn get_created_java_vms(&self) -> Result<Vec<*mut JavaVM>, JniError> {
        let mut vms = Vec::new();
        vms.resize(1, null_mut());

        for _try in 0..10 {
            let n = vms.len() as _;
            let mut nvms = 0;
            let r = unsafe { self.jvm.JNI_GetCreatedJavaVMs(vms.as_mut_ptr(), n, &mut nvms) };
            if r == JNI_OK && nvms <= n {
                vms.resize(nvms as _, null_mut());
                return Ok(vms);
            } else if nvms > n {
                vms.resize(nvms as _, null_mut());
                continue; // retry
            } else {
                return Err(JniError(r));
            }
        }

        Err(JniError(JNI_ENOMEM))
    }

    /// `JNI_GetDefaultJavaVMInitArgs`
    #[cfg_attr(feature = "nightly", doc(cfg(not(target_os = "android"))))] // We actually still compile this in but discourage it as unlikely to work...
    pub fn get_default_java_vm_init_args(&self) -> Result<JavaVMInitArgs, JniError> {
        let mut args : JavaVMInitArgs = JavaVMInitArgs { version: 0, nOptions: 0, options: null_mut(), ignoreUnrecognized: JNI_FALSE };
        let r = unsafe { self.jvm.JNI_GetDefaultJavaVMInitArgs(&mut args as *mut _ as *mut _) };
        if r == JNI_OK {
            Ok(args)
        } else {
            Err(JniError(r))
        }
    }
}
