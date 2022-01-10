//! `%JAVA_HOME%\bin\java` - Run `.jar` files

use std::io::{Error, ErrorKind};
use std::path::{Path};
use std::process::Command;

/// std::io::[Result](https://doc.rust-lang.org/std/io/type.Result.html)
pub type Result<T> = std::io::Result<T>;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Run<'a> {
    pub java_home:          Option<&'a Path>,

    pub files:              &'a [&'a Path],
    pub classpaths:         &'a [&'a Path],
    pub system_properties:  &'a [(&'a str, &'a str)],

    pub verbose_class:      bool,
    pub verbose_gc:         bool,
    pub verbose_jni:        bool,

    pub out_classes:        Option<&'a Path>,
    pub out_sources:        Option<&'a Path>,
    pub out_headers:        Option<&'a Path>,

    pub enable_assertions:         &'a [(&'a str, bool)],
    pub enable_system_assertions:  Option<bool>,

    #[doc(hidden)] pub _non_exhaustive: (),
}

impl<'a> Run<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&self) -> Result<()> {
        let mut java_home_buf = None;
        let java_home = self.java_home.or_else(||{
            java_home_buf = Some(crate::search::find_java_home()?);
            java_home_buf.as_ref().map(|p| &**p)
        }).ok_or_else(||
            Error::new(ErrorKind::NotFound, "JAVA_HOME not set and could not be found, unable to run")
        )?;

        let mut cmd = Command::new(java_home.join("bin/java"));
        // -d32 -d64 -server

        for classpath in self.classpaths    { cmd.arg("-cp").arg(classpath); }
        for (k,v) in self.system_properties { cmd.arg(format!("-D{}={}", k, v)); }

        for (flag, cond) in [
            ("-verbose:class",  self.verbose_class),
            ("-verbose:gc",     self.verbose_gc),
            ("-verbose:jni",    self.verbose_jni),
        ].iter().copied() {
            if cond {
                cmd.arg(flag);
            }
        }

        unimplemented!();
    }
}
