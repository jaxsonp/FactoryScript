use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::process::ExitCode;

use interpreter::*;

fn main() -> ExitCode {
    let cli = Cli::parse();

    // setting global options
    unsafe {
        DEBUG_LEVEL = cli.debug_level;
        COLOR_OUTPUT = !cli.no_color;
        debug!(1, "Debug level:\t{}", DEBUG_LEVEL);
    }

    // reading file
    let file_name: String = match cli.file {
        Some(s) => s,
        None => {
            print_err!("No file provided");
            return ExitCode::FAILURE;
        }
    };
    debug!(1, "Input file:\t{}", file_name);
    let mut file = match File::open(&file_name) {
        Ok(f) => f,
        Err(e) => {
            print_err!("Failed to open file \"{}\": {}", file_name, e);
            return ExitCode::FAILURE;
        }
    };
    debug!(2, "Opened file");
    let mut file_contents = String::new();
    let bytes_read = match file.read_to_string(&mut file_contents) {
        Ok(b) => b,
        Err(e) => {
            print_err!("Failed to read file \"{}\": {}", file_name, e);
            return ExitCode::FAILURE;
        }
    };
    debug!(2, "Read {} bytes", bytes_read);
    debug!(
        4,
        "Contents --------------\n{}\n-----------------------", file_contents
    );

    match run(&file_contents, cli.benchmark) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            print_err!("{}", e.pretty_msg(&file_contents));
            ExitCode::FAILURE
        }
    }
}

// CLI argument parsing struct
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Conveyor program to execute
    file: Option<String>,

    /// Print benchmarking information after completion
    #[arg(short, long)]
    benchmark: bool,

    /// Increase debug logging level, can be supplied multiple times
    #[arg(short = 'd', long = "verbose", action = clap::ArgAction::Count)]
    debug_level: u8,

    /// Disable colored terminal output
    #[arg(long = "no-color")]
    no_color: bool,
}
