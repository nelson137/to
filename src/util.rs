use std::path::PathBuf;

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