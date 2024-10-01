use core::station_types::STATION_TYPES;
use core::StationType;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    let filename = match env::args().nth(1) {
        Some(s) => s,
        None => {
            println!("Please specify an output location");
            return ExitCode::FAILURE;
        }
    };
    let mut file = match File::create(&filename) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file \"{filename}\": {e}");
            return ExitCode::FAILURE;
        }
    };
    println!("Generating station docs...");

    let mut lines = Vec::<String>::new();
    for station_type in STATION_TYPES.iter() {
        lines.push(generate_csv_line(station_type));
    }
    lines.sort_unstable();

    println!("Writing to \"{filename}\"...");

    for line in lines {
        match file.write_all((line + "\n").as_bytes()) {
            Ok(()) => {}
            Err(e) => {
                println!("Failed to write to file: {e}");
            }
        };
    }

    println!("Done");

    return ExitCode::SUCCESS;
}

fn generate_csv_line(station: &StationType) -> String {
    return [
        if station.id == "assign" {
            String::from("``{...}``")
        } else {
            format!("``[{}]``", station.id)
        },
        if station.alt_id.is_some() {
            format!("``[{}]``", station.alt_id.unwrap())
        } else {
            String::new()
        },
        format!("{}", station.inputs),
        if station.output { "yes" } else { "" }.to_string(),
        String::from("description"),
    ]
    .join(",");
}
