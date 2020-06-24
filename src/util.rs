use std::path::{Path, PathBuf};
use std::process::exit;

pub fn die(msg: String) -> ! {
    eprintln!("to: {}", msg);
    exit(1);
}

#[macro_export]
macro_rules! build_path {
    ($($p:expr),+) => (
        {
            let mut path = PathBuf::new();
            $( path.push($p); )*
            path
        }
    )
}

pub trait PathBufAddExtension {
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
    fn is_nopath_exec(&self) -> bool;
}

impl PathIsNopathExec for Path {
    fn is_nopath_exec(&self) -> bool {
        self.is_absolute() || self.starts_with("./") || self.starts_with("../")
    }
}
