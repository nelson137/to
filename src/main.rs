mod compilers;
mod to;
mod util;

use std::fs::remove_file;
use std::path::PathBuf;
use std::process::{exit, Command};
use structopt::StructOpt;

use util::{die, PathBufAddExtension, PathIsNopathExec};

const COMPILE_HELP: &'static str = "Compile <infile> and generate an executable";
const EXECUTE_HELP: &'static str = "Execute the generated executable (requires c)";
const REMOVE_HELP: &'static str = "Remove generated files";
const FORCE_HELP: &'static str = "Force overwrite files if they already exist";
const OUTFILE_HELP: &'static str = "The name for the generated executable";
const INFILE_HELP: &'static str = "The path to the source file";
const ARGS_HELP: &'static str = "The arguments to pass to the execuable";

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long, help=COMPILE_HELP)]
    compile: bool,

    #[structopt(short, long, help=EXECUTE_HELP)]
    execute: bool,

    #[structopt(short, long, help=REMOVE_HELP)]
    remove: bool,

    #[structopt(short, long, help=FORCE_HELP)]
    force: bool,

    #[structopt(short, long, parse(from_os_str), help=OUTFILE_HELP)]
    outfile: Option<PathBuf>,

    #[structopt(parse(from_os_str), help=INFILE_HELP)]
    infile: PathBuf,

    #[structopt(last=true, help=ARGS_HELP)]
    exe_args: Vec<String>,
}

fn main() {
    let mut status = 0;
    let args = Cli::from_args();

    let lang = match to::Lang::determine(&args.infile) {
        Some(l) => l,
        None => die(format!(
            "Language of infile not recognized: {}",
            args.infile.display()
        )),
    };

    let mut generated_files: Vec<PathBuf> = Vec::new();

    let outfile = match args.outfile {
        Some(path) => match path.is_nopath_exec() {
            true => path.clone(),
            false => build_path!(".", path),
        },
        None => match args.infile.file_name() {
            Some(path) => PathBuf::from(path).add_extension("to.exe"),
            None => die(format!("Infile name is invalid: {}", args.infile.display())),
        },
    };

    if args.compile {
        match lang.compile(&args.infile, &outfile) {
            Ok(gfs) => gfs.into_iter().for_each(|f| generated_files.push(f)),
            Err(msg) => die(msg),
        }
    }

    if args.execute {
        let exe_res = Command::new(outfile.as_os_str())
            .args(args.exe_args)
            .status();
        match exe_res {
            Ok(s) => match s.code() {
                Some(code) => status = code,
                None => die(format!(
                    "Executable was terminated by signal: {}",
                    outfile.display()
                )),
            },
            Err(reason) => die(format!(
                "Failed to run executable: {}: {}",
                outfile.display(),
                reason
            )),
        }
    }

    if args.remove {
        for gf in generated_files {
            if let Some(err) = remove_file(&gf).err() {
                die(format!("Failed to remove file: {}: {}", gf.display(), err))
            }
        }
    }

    exit(status);
}
