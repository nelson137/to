use std::path::{Path, PathBuf};
use std::process::exit;

/// Print the given message to the error stream and exit with status code `1`.
pub fn die(msg: String) -> ! {
    eprintln!("to: {}", msg);
    exit(1);
}

/// Build a [`PathBuf`] with the given components.
/// This will create a new [`PathBuf`] and call [`new`] on it with each component.
///
/// [`PathBuf`]: struct.PathBuf.html
/// [`new`]: struct.PathBuf.html#method.new
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// assert_eq!(build_path!("/", "bin", "touch"), PathBuf::from("/bin/touch"));
/// ```
#[macro_export]
macro_rules! build_path {
    ($($component:expr),+) => ({
            let mut path = PathBuf::new();
        $( path.push($component); )*
            path
    })
        }

pub trait PathBufAddExtension {
    /// Return an owned `Path` with the given extension appended.
    ///
    /// Similar to [`self.set_extension`] in that the extension is added if
    /// none exists.  Except if one already exists, the given extension is
    /// appended.
    ///
    /// [`self.set_extension`]: struct.Path.html#method.set_extension
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use crate::util::PathBufAddExtension;
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
}

impl PathBufAddExtension for PathBuf {
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
}

pub trait PathIsNopathExec {
    /// Return whether the `Path` can be executed if it is not found in `$PATH`.
    ///
    /// This is true iff the `Path` is:
    /// - absolute
    /// - relative and specifies the directory, i.e. it starts with either
    /// `./` or `../`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use crate::util::PathIsNopathExec
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
}

impl PathIsNopathExec for Path {
    fn is_nopath_exec(&self) -> bool {
        self.is_absolute() || self.starts_with("./") || self.starts_with("../")
    }
}
