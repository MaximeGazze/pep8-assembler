use assembler::assemble;
use clap::Parser;
use lexer::parse_file;
use std::{fs::File, io::Write, path::PathBuf};

mod address;
mod assembler;
mod dotcommand;
mod instruction;
mod lexer;
mod register;
mod types;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CommandLineArguments {
    /// Assembler source file
    #[arg(required = true)]
    input_file: PathBuf,

    /// Assembler output file
    #[arg(short, long)]
    output_file: Option<PathBuf>,
}

fn main() {
    let args = CommandLineArguments::parse();

    let r = parse_file(args.input_file).unwrap();

    for l in &r {
        println!("{:?}", l);
    }

    let byte_code = assemble(r).unwrap();

    match args.output_file {
        None => println!("{:02X?}", byte_code),
        Some(output_file_path) => {
            let mut output_file = File::create(output_file_path).unwrap();
            output_file.write(&byte_code).unwrap();
        }
    }
}
