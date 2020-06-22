use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

pub fn asm(infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut obj_file = infile.clone();
    obj_file.set_extension("o");

    let output = Command::new("nasm")
        .arg("-f").arg("elf64")
        .arg(infile.clone().into_os_string())
        .arg("-o").arg(obj_file.clone().into_os_string())
        .output();
    let output = match output {
        Ok(o) => o,
        Err(_) => return Err("Failed to execute nasm".to_string())
    };

    if output.status.success() == false {
        io::stderr().write_all(&output.stderr).unwrap();
        return Err(format!("Failed to assemble infile: {}", infile.display()));
    }

    let output = Command::new("ld")
        .arg(obj_file.clone().into_os_string())
        .arg("-o").arg(outfile.clone().into_os_string())
        .output();
    let output = match output {
        Ok(o) => o,
        Err(_) => return Err("Failed to execute ld".to_string())
    };

    if output.status.success() {
        Ok(vec![obj_file.clone(), outfile.clone()])
    } else {
        io::stderr().write_all(&output.stderr).unwrap();
        Err(format!("Failed to link object file: {}", obj_file.display()))
    }
}

pub fn c(infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let output = Command::new("gcc")
        .arg(infile.clone().into_os_string())
        .arg("-o").arg(outfile.clone().into_os_string())
        .output();
    let output = match output {
        Ok(o) => o,
        Err(_) => return Err("Failed to execute gcc".to_string())
    };

    if output.status.success() {
        Ok(vec![outfile.clone()])
    } else {
        io::stderr().write_all(&output.stderr).unwrap();
        Err(format!("Failed to compile infile: {}", infile.display()))
    }
}

pub fn cpp(infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let output = Command::new("g++")
        .arg(infile.clone().into_os_string())
        .arg("-o").arg(outfile.clone().into_os_string())
        .output();
    let output = match output {
        Ok(o) => o,
        Err(_) => return Err("Failed to execute g++".to_string())
    };

    if output.status.success() {
        Ok(vec![outfile.clone()])
    } else {
        io::stderr().write_all(&output.stderr).unwrap();
        Err(format!("Failed to compile infile: {}", infile.display()))
    }
}
