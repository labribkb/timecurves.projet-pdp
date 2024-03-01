use std::{path::PathBuf, process::exit};

use clap::Parser;

use timecurves_rs::input::InputData;

#[derive(Parser)]
struct CommandLine {
    /// Specifies the input file for generating the curves.
    /// The file must be in the correct JSON format, as per the provided template.
    input: PathBuf,
    /// Print additional debug information to the standard output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cmd = CommandLine::parse();

    let filename = cmd.input.display().to_string();

    let input: InputData = match InputData::from_filename(&filename) {
        Ok(v) => v,
        Err(e) => {
            println!("Error while parsing the input file :");
            println!("{}", e);
            exit(1);
        }
    };

    if cmd.verbose {
        println!("Input file <{}> read.", &cmd.input.display());
        println!("Contains {} datasets :", input.data.len());
        for dataset in input.data {
            println!("  - {}", dataset.name);
        }
    }
}
