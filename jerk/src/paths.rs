//! JDK and JVM path utilities

// https://developer.android.com/studio/command-line/variables
// https://github.com/MaulingMonkey/jerk/wiki/Java-Paths



use std::convert::AsRef;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Read an environment variable as a path
pub fn env(var: &str) -> Option<PathBuf> {
    std::env::var_os(var).map(|os| os.into())
}

/// Return `%ANDROID_SDK_ROOT%\platforms\android-29\` or similar path.
/// 
/// Expected contents:
/// * android.jar
/// 
/// ```rust
/// # if std::env::var_os("CI").is_none() {
/// let android_sdk_root = jerk::paths::android_sdk_root().unwrap();
/// let android_nn = jerk::paths::platforms_android_nn(&android_sdk_root).unwrap();
/// 
/// assert!(android_nn.join("android.jar").exists());
/// # }
/// ```
pub fn platforms_android_nn(android_sdk_root: &impl AsRef<Path>) -> Result<PathBuf, io::Error> {
    let android_sdk_root = android_sdk_root.as_ref();
    if_exists_any(&android_sdk_root.join("platforms"), "android-*")
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, format!("No android-NN platform found in Android SDK root: {}/platforms/android-*", android_sdk_root.display())))
}

/// Return `%ANDROID_SDK_ROOT%` or similar path.
/// 
/// Possible contents (varies wildly based on what's installed):
/// * build-tools/29.0.2/{aapt,aapt2,apksigner,lib,\*-linux-android\*-ld}
/// * ndk-bundle/ndk-build
/// * ndk-bundle/ndk-gdb
/// * ndk-bundle/platforms/android-29/arch-{arm,arm64,x86,x86_64}/usr/lib/lib{EGL,GLESv2,GLESv3,...}.so
/// * platform-tools/
/// * platforms/android-29/android.jar
/// * tools/
pub fn android_sdk_root() -> Result<PathBuf, io::Error> {
    if let Some(android_home) = env("ANDROID_HOME") {
        Some(android_home)
    } else if let Some(android_sdk_root) = env("ANDROID_SDK_ROOT") {
        Some(android_sdk_root)
    } else if cfg!(windows) {
        let WinPaths { program_files: _, program_files_x86, local_app_data } = WinPaths::get();
        None.or_else(|| if_exists(program_files_x86.join(r"Android\android-sdk")))
            .or_else(|| if_exists(local_app_data.join(r"Android\Sdk")))
    } else if cfg!(unix) {
        let home = env("HOME").expect("Expected ${HOME} to be set");
        if_exists(home.join("android-sdk-tmp"))
    } else {
        None
    }
    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "ANDROID_HOME nor ANDROID_SDK_ROOT set and no Android SDK installation could be found"))
}

/// Return `%JAVA_HOME%` or similar path.
/// 
/// Expected windows contents:
/// * bin\java.exe
/// * bin\javac.exe
/// * bin\javadoc.exe
/// * bin\javah.exe
/// 
/// Expected unix contents:
/// * bin/java
/// * bin/javac
/// * bin/javadoc
/// * bin/javah
/// 
/// ```rust
/// let java_home = jerk::paths::java_home().unwrap();
/// let bin = java_home.join("bin");
/// 
/// if cfg!(windows) {
///     assert!(bin.join("java.exe").exists());
///     assert!(bin.join("javac.exe").exists());
///     assert!(bin.join("javadoc.exe").exists());
///     assert!(bin.join("javah.exe").exists());
/// } else {
///     assert!(bin.join("java").exists());
///     assert!(bin.join("javac").exists());
///     assert!(bin.join("javadoc").exists());
///     assert!(bin.join("javah").exists());
/// }
/// ```
pub fn java_home() -> Result<PathBuf, io::Error> {
    if let Some(java_home) = env("JAVA_HOME") {
        Some(java_home)
    } else if cfg!(windows) {
        let WinPaths { program_files, program_files_x86, local_app_data: _ } = WinPaths::get();
        None.or_else(|| if_exists_any(&program_files_x86.join(r"Java"), "jdk*"))
            .or_else(|| if_exists_any(&program_files.join(r"Android\jdk"), "microsoft_disk_openjdk_*"))
            .or_else(|| if_exists(program_files.join(r"Android\Android Studio\jre")))
    } else if cfg!(unix) {
        if_exists_any("/usr/lib/jvm", "java-*-openjdk-amd64")
    } else {
        None
    }
    .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "JAVA_HOME not set and no Java installation could be found"))
}

/// Return `%JAVA_HOME%\jre\bin\client\` or similar path.
/// 
/// Expected contents:
/// * jvm.dll (windows)
/// * libjvm.so (unix)
/// 
/// ```rust
/// let java_home   = jerk::paths::java_home().unwrap();
/// let jvm_dir     = jerk::paths::libjvm_dir(&java_home).unwrap();
/// 
/// if cfg!(windows) {
///     assert!(jvm_dir.join("jvm.dll").exists());
/// } else {
///     assert!(jvm_dir.join("libjvm.so").exists());
/// }
/// ```
pub fn libjvm_dir(java_home: &impl AsRef<Path>) -> Result<PathBuf, io::Error> {
    let java_home = java_home.as_ref();
    let libjvm = if cfg!(windows) { "jvm.dll" } else { "libjvm.so" };
    for path in [
        // TODO: Make it possible to indicate preference instead of prioritizing client
        "jre/bin/client",
        "jre/lib/amd64/client",
        "jre/bin/server",
        "jre/lib/amd64/server"
    ].iter().copied().map(|s| Path::new(s)) {
        let path = java_home.join(path);
        if path.join(libjvm).exists() {
            return Ok(path);
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, format!("Could not find {} in expected locations of JAVA_HOME: {}/jre/{{bin,lib/amd64}}/{{client,server}}/", libjvm, java_home.display())))
}



fn for_each_dir<T>(dir: &Path, pattern: &str, on_dir: &mut impl FnMut(PathBuf, &str) -> Option<T>) -> Option<T> {
    if let Some(star) = pattern.find('*') {
        let (pre, post) = pattern.split_at(star);
        let post = &post[1..];
        if let Ok(dir) = fs::read_dir(dir) { // XXX: We ignore errors here (missing dirs)
            for entry in dir {
                let entry = if let Ok(e) = entry { e } else { continue }; // XXX: Ignored error (invalid file entry)
                let name = entry.file_name();
                let name = if let Some(n) = name.to_str() { n } else { continue }; // XXX: Ignored error (invalid unicode in file name)

                if name.starts_with(pre) && name.ends_with(post) {
                    let ver = &name[pre.len()..name.len()-post.len()];
                    if let Some(r) = on_dir(entry.path(), ver) {
                        return Some(r);
                    }
                }
            }
        }
        None
    } else {
        let dir = dir.join(pattern);
        if !dir.exists() { return None; }
        on_dir(dir, "")
    }
}

fn if_exists_any(dir: &(impl AsRef<Path> + ?Sized), pattern: &str) -> Option<PathBuf> {
    for_each_dir(dir.as_ref(), pattern, &mut |p,_v| Some(p))
}

fn if_exists<P: AsRef<Path>>(path: P) -> Option<P> {
    if path.as_ref().exists() {
        Some(path)
    } else {
        None
    }
}

struct WinPaths {
    program_files:      PathBuf,
    program_files_x86:  PathBuf,
    local_app_data:     PathBuf,
}

impl WinPaths {
    pub fn get() -> Self {
        Self {
            program_files:      env("ProgramW6432"      ).or_else(|| env("ProgramFiles")) .expect("Expected %ProgramW6432% or %ProgramFiles% to be set"),
            program_files_x86:  env("ProgramFiles(x86)" ).or_else(|| env("ProgramFiles")) .expect("Expected %ProgramW6432% or %ProgramFiles% to be set"),
            local_app_data:     env("LOCALAPPDATA"      )                                           .expect("Expected %LOCALAPPDATA% to be set"),
        }
    }
}
