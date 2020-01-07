use crate::env::*;
use std::path::PathBuf;

/// Find a suitable `%JAVA_HOME%`
pub fn find_java_home() -> Option<PathBuf> {
    None
        .or_else(|| var_path("JAVA_HOME"))
        .or_else(|| var_path("ProgramW6432").map(|p| p.join("Android\\Android Studio\\jre")))
        .or_else(|| var_path("ProgramFiles").map(|p| p.join("Android\\Android Studio\\jre")))
}
