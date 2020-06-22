use std::path::PathBuf;

use crate::compilers;

use to::SupportedLangs;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, SupportedLangs)]
pub enum Lang {
    #[compile_func(compilers::asm)]
    Asm,

    #[compile_func(compilers::c)]
    #[extensions("c", "cc")]
    C,

    #[compile_func(compilers::cpp)]
    #[extensions("cpp")]
    Cpp
}
