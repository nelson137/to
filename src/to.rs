use std::path::PathBuf;

use crate::compilers;

use to::SupportedLangs;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, SupportedLangs)]
pub enum Lang {
    #[compile_func(compilers::asm)]
    #[extensions("s", "S", "asm")]
    Asm,

    #[compile_func(compilers::c)]
    #[extensions("c")]
    C,

    #[compile_func(compilers::cpp)]
    #[extensions("cpp", "cxx", "CXX")]
    Cpp,
}
