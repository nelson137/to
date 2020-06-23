use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

struct CompileStep {
    bin: String,
    outfile: PathBuf,
    command: Command,
}

impl CompileStep {
    fn new(bin: &str) -> Self {
        Self {
            bin: String::from(bin),
            outfile: PathBuf::new(),
            command: Command::new(bin),
        }
    }

    fn arg<S: AsRef<OsStr>>(&mut self, a: S) -> &mut Self {
        self.command.arg(a);
        self
    }

    fn arg_outfile(&mut self, o: &PathBuf) -> &mut Self {
        self.outfile = o.clone();
        self.command.arg(o.as_os_str());
        self
    }

    fn execute_with_err(&mut self, err_msg: String) -> Result<Vec<PathBuf>, String> {
        match self.command.status() {
            Ok(status) => {
                if status.success() {
                    Ok(vec![self.outfile.clone()])
                } else {
                    Err(err_msg)
                }
            }
            Err(e) => Err(format!("Failed to execute {}: {}", self.bin, e)),
        }
    }
}

pub fn asm(infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut gen_files: Vec<PathBuf> = Vec::new();

    let mut obj_file = outfile.clone();
    obj_file.set_extension("o");

    CompileStep::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg(infile.as_os_str())
        .arg("-o")
        .arg_outfile(&obj_file)
        .execute_with_err(format!("Failed to assemble infile: {}", infile.display()))?
        .iter()
        .for_each(|gf| gen_files.push(gf.clone()));

    CompileStep::new("ld")
        .arg(obj_file.as_os_str())
        .arg("-o")
        .arg_outfile(outfile)
        .execute_with_err(format!(
            "Failed to link object file: {}",
            obj_file.display()
        ))?
        .iter()
        .for_each(|gf| gen_files.push(gf.clone()));

    Ok(gen_files)
}

pub fn c(infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
    CompileStep::new("gcc")
        .arg(infile.as_os_str())
        .arg("-o")
        .arg_outfile(outfile)
        .execute_with_err(format!("Failed to compile infile: {}", infile.display()))
}

pub fn cpp(infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
    CompileStep::new("g++")
        .arg(infile.as_os_str())
        .arg("-o")
        .arg_outfile(outfile)
        .execute_with_err(format!("Failed to compile infile: {}", infile.display()))
}

pub fn rust(infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
    CompileStep::new("rustc")
        .arg(infile.as_os_str())
        .arg("-o")
        .arg_outfile(outfile)
        .execute_with_err(format!("Failed to compile infile: {}", infile.display()))
}
