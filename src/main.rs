use clap::Parser;
use minclang_rust::codegen::codegen;
use minclang_rust::compiler::{compile, parse, tokenize};
use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit};

#[derive(Parser)]
#[command(name = "minclang", about = "Compile an arithmetic expression to a native binary")]
struct Opts {
    #[arg(help = "Source file to compile")]
    file: PathBuf,

    #[arg(short = 'o', default_value = "out", help = "Output file name")]
    output: String,

    #[arg(short = 'S', help = "Save assembly source to FILE")]
    asm_file: Option<PathBuf>,
}

fn main() {
    let opts = Opts::parse();

    let source = match fs::read_to_string(&opts.file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };

    let ast = match tokenize(&source).and_then(parse) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };

    let asm = codegen(&compile(&ast));

    let asm_path = match &opts.asm_file {
        Some(path) => {
            fs::write(path, &asm).unwrap_or_else(|e| {
                eprintln!("Error writing assembly: {e}");
                exit(1);
            });
            path.clone()
        }
        None => {
            let tmp = std::env::temp_dir().join("minclang.s");
            fs::write(&tmp, &asm).unwrap_or_else(|e| {
                eprintln!("Error writing assembly: {e}");
                exit(1);
            });
            tmp
        }
    };

    let status = Command::new("gcc")
        .arg(&asm_path)
        .arg("-o")
        .arg(&opts.output)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("Error running gcc: {e}");
            exit(1);
        });

    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}
