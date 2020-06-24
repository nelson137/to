use std::path::PathBuf;
use std::process::exit;

/// Print the given message to the error stream and exit with status code `1`.
pub fn die(msg: String) -> ! {
    eprintln!("to: {}", msg);
    exit(1);
}

pub trait PathBufUtils {
    /// Return an owned `PathBuf` with the given extension appended.
    ///
    /// Similar to [`self.set_extension`] in that the extension is added if
    /// none exists.  Except if one already exists, the given extension is
    /// appended.
    ///
    /// [`self.set_extension`]: struct.PathBuf.html#method.set_extension
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use crate::util::PathBufUtils;
    ///
    /// let a = PathBuf::from("file").add_extension("sh");
    /// assert_eq!(a, PathBuf::from("file.sh"));
    /// assert_eq!(a.file_stem().unwrap(), "file");
    /// assert_eq!(a.extension().unwrap(), "sh");
    ///
    /// let b = PathBuf::from("file2.tar").add_extension(".xz")
    /// assert_eq!(b, PathBuf::from("file2.tar.xz"));
    /// assert_eq!(b.file_stem().unwrap(), "file2.tar");
    /// assert_eq!(b.extension().unwrap(), "xz");
    /// ```
    fn add_extension(&mut self, ext: &str) -> Self;

    /// Return whether the `PathBuf` can be executed if it is not found in `$PATH`.
    ///
    /// This is true iff the `PathBuf` is:
    /// - absolute
    /// - relative and specifies the directory, i.e. it starts with either
    /// `./` or `../`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use crate::util::PathBufUtils
    ///
    /// let a = PathBuf::from("examples/t.c");
    /// let b = PathBuf::from("./a.out");
    /// let c = PathBuf::from("/bin/sh");
    ///
    /// assert_eq!(a.is_nopath_exec(), false);
    /// assert_eq!(b.is_nopath_exec(), true);
    /// assert_eq!(c.is_nopath_exec(), true);
    /// ```
    fn is_nopath_exec(&self) -> bool;

    /// Creates an owned `PathBuf` like `self` that is executable if not in `$PATH`.
    ///
    /// If [`self.is_noexec_path()`] is `true` it is a clone, otherwise `./` is prepended.
    ///
    /// [`self.is_noexec_path()`]: struct.PathBuf.html#method.is_nopath_exec
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use crate::util::PathBufUtils
    ///
    /// let p = PathBuf::from("/bin/ls");
    /// assert_eq!(p.to_nopath_exec(), p);
    ///
    /// assert_eq!(PathBuf::from("a.out").to_nopath_exec(), PathBuf::from("./a.out"));
    /// ```
    fn to_nopath_exec(&self) -> Self;
}

impl PathBufUtils for PathBuf {
    fn add_extension(&mut self, ext: &str) -> Self {
        let ext = String::from(ext);
        self.with_extension(match self.extension() {
            None => ext,
            Some(old_ext) => match old_ext.to_str() {
                None => ext,
                Some(old_ext) => format!("{}.{}", old_ext, ext),
            },
        })
    }

    fn is_nopath_exec(&self) -> bool {
        self.is_absolute() || self.starts_with("./") || self.starts_with("../")
    }

    fn to_nopath_exec(&self) -> Self {
        if self.is_nopath_exec() {
            self.clone()
        } else {
            let mut p = PathBuf::from(".");
            p.push(self);
            p
        }
    }
}
