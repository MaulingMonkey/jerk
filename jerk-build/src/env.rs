pub use std::env::*;
use std::path::PathBuf;

pub fn var_path(var: &str) -> Option<PathBuf> {
    var_os(var).map(|os| os.into())
}
