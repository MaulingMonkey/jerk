use crate::paths::env;
use std::path::PathBuf;

/// A list of known JVM architectures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Arch {
    X86_64,
    X86,
    AArch64,
    ARM,

    #[allow(non_camel_case_types)] #[doc(hidden)] _non_exhaustive,
}

#[allow(non_upper_case_globals)]
impl Arch {
    pub const x86_64 : Arch = Arch::X86_64;
    pub const x86    : Arch = Arch::X86;

    /// The architecture the current Rust module is built for.  Note that in a
    /// build script, this is the *host* architecture / the architecture of the
    /// build script, *not* the architecture of the final target!
    pub const Host : Arch = Self::_Host;
    #[cfg(target_arch = "x86_64" )] const _Host : Arch = Arch::X86_64;
    #[cfg(target_arch = "x86"    )] const _Host : Arch = Arch::X86;
    #[cfg(target_arch = "aarch64")] const _Host : Arch = Arch::AArch64;
    #[cfg(target_arch = "arm"    )] const _Host : Arch = Arch::ARM;

    // We could read CARGO_CFG_TARGET_ARCH from build.rs scripts, but note that
    // this isn't exposed to proc macros!  I... don't really have a good option
    // there I don't think.

    /// Return the "Program Files" directory for the specified architecture,
    /// if available on this computer - or if it *should* be.
    pub fn program_files(self) -> Option<PathBuf> {
        if cfg!(windows) {
            match self {
                Arch::X86_64    => env("ProgramW6432"     ).or_else(|| env("ProgramFiles")),
                Arch::X86       => env("ProgramFiles(x86)").or_else(|| env("ProgramFiles")),
                _other          => None,
            }
        } else {
            None
        }
    }
}
