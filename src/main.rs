mod compilers;
mod to;

use std::fs::remove_file;
use std::path::PathBuf;
use std::process::{Command, exit};
use structopt::StructOpt;

static COMPILE_HELP: &'static str =
"Compile <infile> and generate an executable";
static EXECUTE_HELP: &'static str =
"Execute the generated executable (requires c)";
static REMOVE_HELP: &'static str =
"Remove generated files";
static FORCE_HELP: &'static str =
"Force overwrite files if they already exist";
static DRYRUN_HELP: &'static str =
"Print the commands that would be executed then exit";
static OUTFILE_HELP: &'static str =
"The name for the generated executable";
static INFILE_HELP: &'static str =
"The path to the source file";
static ARGS_HELP: &'static str =
"The arguments to pass to the execuable";

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

    #[structopt(short, long, help=DRYRUN_HELP)]
    dry_run: bool,

    #[structopt(short, long, parse(from_os_str), help=OUTFILE_HELP)]
    outfile: Option<PathBuf>,

    #[structopt(parse(from_os_str), help=INFILE_HELP)]
    infile: PathBuf,

    #[structopt(last=true, help=ARGS_HELP)]
    exe_args: Vec<String>,
}

fn die(msg: String) -> ! {
    eprintln!("to: {}", msg);
    exit(1);
}

fn build_outfile_name(infile: &PathBuf) -> PathBuf {
    infile.with_extension(match infile.extension() {
        None => String::from("o"),
        Some(e) => match e.to_str() {
            None => String::from("o"),
            Some(e) => String::from(format!("{}.o", e))
        }
    })
}

fn main() {
    let mut status = 0;
    let args = Cli::from_args();

    let lang = match to::Lang::determine(&args.infile) {
        Some(l) => l,
        None => die(format!(
            "Language of infile not recognized: {}",
            args.infile.display()))
    };

    let mut generated_files: Vec<PathBuf> = Vec::new();

    let outfile = match args.outfile {
        Some(path) => path,
        None => build_outfile_name(&args.infile)
    };
    let outfile_abs = if outfile.is_relative() {
        PathBuf::from(format!("./{}", outfile.to_str().unwrap()))
    } else {
        outfile.clone()
    };

    if args.compile {
        match lang.compile(&args.infile, &outfile) {
            Ok(gfs) => gfs.into_iter().for_each(|f| generated_files.push(f)),
            Err(msg) => die(msg)
        }
    }

    if args.execute {
        let exe_res = Command::new(outfile_abs.as_os_str())
            .args(args.exe_args)
            .status();
        if exe_res.is_err() {
            die(format!("Failed to run executable: {}", outfile_abs.display()))
        }
        match exe_res.unwrap().code() {
            Some(code) => status = code,
            None => die(format!("Executable was terminated by signal"))
        }
    }

    if args.remove {
        for gf in generated_files {
            if gf.exists() {
                if remove_file(&gf).is_err() {
                    die(format!(
                        "Failed to remove file: {}",
                        gf.display()))
                }
            } else {
                die(format!(
                    "Cannot remove file that does not exist: {}",
                    gf.display()));
            }
        }
    }

    exit(status);
}
