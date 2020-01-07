//! `%JAVA_HOME%\bin\jar` - Create `.jar` files from `.class` files
//! 
//! | Command       | Description                   | API |
//! | ------------- | ----------------------------- | --- |
//! | `jar c...`    | Create `.jar`                 | `jar::Archive{ ... }.create()`
//! | `jar u...`    | Update `.jar`                 | `jar::Archive{ ... }.update()`
//! | `jar x...`    | Extract `.jar`                | *NYI*
//! | `jar t...`    | List `.jar` table of contents | *NYI*
//! | `jar i...`    | Generate `.jar` index         | *NYI*

use std::io::{Error, ErrorKind};
use std::path::{Path};
use std::process::Command;

/// std::io::[Result](https://doc.rust-lang.org/std/io/type.Result.html)
pub type Result<T> = std::io::Result<T>;

/// Create or update a `.jar` file
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Archive<'a> {
    pub java_home:      Option<&'a Path>,

    pub jar_file:       Option<&'a Path>,
    pub manifest_file:  Option<&'a Path>,
    pub entry_point:    Option<String>,

    pub verbose:                        bool,
    pub pack200_normalization:          bool,
    pub uncompressed:                   bool,
    pub preserve_original_filenames:    bool,
    pub no_manifest:                    bool,

    pub files: &'a [(&'a Path, &'a [&'a Path])],

    #[doc(hidden)] pub _non_exhaustive: (),
}

impl<'a> std::default::Default for Archive<'a> {
    fn default() -> Self {
        Self {
            java_home:          None,

            jar_file:           None,
            manifest_file:      None,
            entry_point:        None,

            verbose:                        false,
            pack200_normalization:          false,
            uncompressed:                   false,
            preserve_original_filenames:    false,
            no_manifest:                    false,

            files: &[][..],

            _non_exhaustive:    (),
        }
    }
}

impl<'a> Archive<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new archive
    /// 
    /// Executes: `jar c...`
    pub fn create(&self) -> Result<()> {
        self.exec('c')
    }

    /// Update an existing archive
    /// 
    /// Executes: `jar u...`
    pub fn update(&self) -> Result<()> {
        self.exec('u')
    }

    fn exec(&self, create_or_update: char) -> Result<()> {
        let mut java_home_buf = None;
        let java_home = self.java_home.or_else(||{
            java_home_buf = Some(crate::search::find_java_home()?);
            java_home_buf.as_ref().map(|p| &**p)
        }).ok_or_else(||
            Error::new(ErrorKind::NotFound, "JAVA_HOME not set and could not be found, unable to run")
        )?;

        let mut flags_arg = String::new();
        for (flag, cond) in [
            (create_or_update, true),
            ('v', self.verbose),
            ('f', self.jar_file.is_some()),
            ('m', self.manifest_file.is_some()),
            ('n', self.pack200_normalization),
            ('e', self.entry_point.is_some()),
            ('0', self.uncompressed),
            ('P', self.preserve_original_filenames),
            ('M', self.no_manifest),
        ].iter().copied() {
            if cond {
                flags_arg.push(flag);
            }
        }

        let mut cmd = Command::new(java_home.join("bin/jar"));
        cmd.arg(&flags_arg);

        if let Some(jar_file)       = self.jar_file.as_ref()        { cmd.arg(jar_file); }
        if let Some(manifest_file)  = self.manifest_file.as_ref()   { cmd.arg(manifest_file); }
        if let Some(entry_point)    = self.entry_point.as_ref()     { cmd.arg(entry_point); }

        for (dir, files) in self.files {
            cmd.arg("-C").arg(dir);
            for file in *files {
                cmd.arg(file);
            }
        }

        let status = cmd.status()?;
        if status.success() {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, format!("jar {} ... failed: {:?}", &flags_arg, status)))
        }
    }
}
