use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

struct CompileStep {
    bin: String,
    outfile: PathBuf,
    command: Command,
    error_message: String,
}

impl CompileStep {
    fn new(bin: &str) -> Self {
        Self {
            bin: String::from(bin),
            outfile: PathBuf::new(),
            command: Command::new(bin),
            error_message: String::from(""),
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

    fn err_msg(&mut self, msg: String) -> &mut Self {
        self.error_message = msg;
        self
    }

    fn execute(&mut self) -> Result<Vec<PathBuf>, String> {
        let output = match self.command.output() {
            Ok(o) => o,
            Err(e) => return Err(format!("Failed to execute {}: {}", self.bin, e)),
        };

        if output.status.success() {
            Ok(vec![self.outfile.clone()])
        } else {
            io::stderr().write_all(&output.stderr).unwrap();
            Err(self.error_message.clone())
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
        .err_msg(format!("Failed to assemble infile: {}", infile.display()))
        .execute()?
        .into_iter()
        .for_each(|gf| gen_files.push(gf));

    CompileStep::new("ld")
        .arg(obj_file.as_os_str())
        .arg("-o")
        .arg_outfile(outfile)
        .err_msg(format!(
            "Failed to link object file: {}",
            obj_file.display()
        ))
        .execute()?
        .into_iter()
        .for_each(|gf| gen_files.push(gf));

    Ok(gen_files)
}

pub fn c(infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
    CompileStep::new("gcc")
        .arg(infile.as_os_str())
        .arg("-o")
        .arg_outfile(outfile)
        .err_msg(format!("Failed to compile infile: {}", infile.display()))
        .execute()
}

pub fn cpp(infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
    CompileStep::new("g++")
        .arg(infile.as_os_str())
        .arg("-o")
        .arg_outfile(outfile)
        .err_msg(format!("Failed to compile infile: {}", infile.display()))
        .execute()
}

pub fn rust(infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
    CompileStep::new("rustc")
        .arg(infile.as_os_str())
        .arg("-o")
        .arg_outfile(outfile)
        .err_msg(format!("Failed to compile infile: {}", infile.display()))
        .execute()
}
