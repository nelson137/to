use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

pub fn asm(infile: &PathBuf, outfile: &PathBuf) -> Result<Vec<PathBuf>, String> {
    println!("nasm {} -o {}", infile.display(), outfile.display());

    Ok(vec![outfile.clone()])
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
