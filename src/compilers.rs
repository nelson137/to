use std::path::PathBuf;

pub fn asm(infile: &PathBuf, outfile: &PathBuf) {
    println!("nasm {} -o {}", infile.display(), outfile.display());
}

pub fn c(infile: &PathBuf, outfile: &PathBuf) {
    println!("gcc {} -o {}", infile.display(), outfile.display());
}

pub fn cpp(infile: &PathBuf, outfile: &PathBuf) {
    println!("g++ {} -o {}", infile.display(), outfile.display());
}
