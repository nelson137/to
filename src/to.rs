use std::path::PathBuf;

use crate::compilers;

use to::ToLangs;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, ToLangs)]
pub enum Lang {
    #[to_lang(compile_func=compilers::asm, extensions("s", "S", "asm"))]
    Asm,

    #[to_lang(compile_func=compilers::c, extensions("c"))]
    C,

    #[to_lang(compile_func=compilers::cpp, extensions("cpp", "cxx", "CXX"))]
    Cpp,

    #[to_lang(compile_func=compilers::rust, extensions("rs"))]
    Rust,
}
