//! `%JAVA_HOME%\bin\javac` - Compile `.class` files from `.java` files

use std::io::{Error, ErrorKind};
use std::path::{Path};
use std::process::Command;

/// std::io::[Result](https://doc.rust-lang.org/std/io/type.Result.html)
pub type Result<T> = std::io::Result<T>;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Compile<'a> {
    pub java_home:                  Option<&'a Path>,
    pub debug_info:                 Option<DebugInfo>,
    pub nowarn:                     bool,
    pub verbose:                    bool,
    pub deprecation:                bool,
    pub class_paths:                &'a [&'a Path],
    pub source_paths:               &'a [&'a Path],
    pub boot_class_paths:           &'a [&'a Path],
    pub extension_dirs:             &'a [&'a Path],
    pub endorsed_dirs:              &'a [&'a Path],
    // -proc:{none,only}
    pub annotation_processors:      &'a [&'a str],
    pub annotation_processor_paths: &'a [&'a Path],
    pub keep_parameter_names:       bool,
    pub out_classes:                Option<&'a Path>,
    pub out_sources:                Option<&'a Path>,
    pub out_headers:                Option<&'a Path>,
    // -implicit
    // -encoding
    // -source <release>
    // -target <release>
    // -profile <profile>
    // -version
    // -help
    pub annotation_parameters:      &'a [(&'a str, &'a str)],
    pub fatal_warnings:             bool,

    pub files:                      &'a [&'a Path],

    #[doc(hidden)] pub _non_exhaustive: (),
}

bitflags::bitflags! {
    pub struct DebugInfo : u32 {
        const NONE      = 0;
        const LINES     = (1 << 0);
        const VARS      = (1 << 1);
        const SOURCE    = (1 << 2);
        const ALL       = 0xFFFFFFFF;
    }
}

impl std::default::Default for DebugInfo {
    fn default() -> Self { Self::ALL }
}

impl<'a> Compile<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn exec(&self) -> Result<()> {
        let status = self.command()?.status()?;
        if status.success() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, format!("javac ... failed: {:?}", status)))
        }
    }

    pub fn command(&self) -> Result<Command> {
        let mut java_home_buf = None;
        let java_home = self.java_home.or_else(||{
            java_home_buf = Some(crate::search::find_java_home()?);
            java_home_buf.as_ref().map(|p| &**p)
        }).ok_or_else(||
            Error::new(ErrorKind::NotFound, "JAVA_HOME not set and could not be found, unable to run")
        )?;

        let mut cmd = Command::new(java_home.join("bin").join("javac"));
        match self.debug_info {
            None => {},
            Some(DebugInfo::NONE)   => { cmd.arg("-g:none"); },
            Some(DebugInfo::ALL)    => { cmd.arg("-g"); },
            Some(debug_info) => {
                for (flag, di) in [
                    ("-g:lines",    DebugInfo::LINES),
                    ("-g:vars",     DebugInfo::VARS),
                    ("-g:source",   DebugInfo::SOURCE),
                ].iter().copied() {
                    if debug_info.contains(di) {
                        cmd.arg(flag);
                    }
                }
            }
        }

        for p in self.class_paths           { cmd.arg("-cp").arg(p); }
        for p in self.source_paths          { cmd.arg("-sourcepath").arg(p); }
        for p in self.boot_class_paths      { cmd.arg("-bootclasspath").arg(p); }
        for p in self.extension_dirs        { cmd.arg("-extdirs").arg(p); }

        let processors = self.annotation_processors.join(",");
        if processors.len() != 0 { cmd.arg("-processors").arg(processors); }

        for p in self.annotation_processor_paths { cmd.arg("-processorpath").arg(p); }

        for (flag, dir) in [
            ("-d", self.out_classes.as_ref()),
            ("-s", self.out_sources.as_ref()),
            ("-h", self.out_headers.as_ref()),
        ].iter().copied() {
            if let Some(dir) = dir {
                cmd.arg(flag).arg(dir);
            }
        }

        for (flag, cond) in [
            ("-nowarn",         self.nowarn),
            ("-verbose",        self.verbose),
            ("-deprecation",    self.deprecation),
            ("-parameters",     self.keep_parameter_names),
            ("-Werror",         self.fatal_warnings),
        ].iter().copied() {
            if cond { cmd.arg(flag); }
        }

        for (k,v) in self.annotation_parameters { cmd.arg(format!("-A{}={}", k, v)); }
        for file in self.files { cmd.arg(file); }

        Ok(cmd)
    }
}
