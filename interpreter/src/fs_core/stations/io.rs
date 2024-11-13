use std::io::{stdin, stdout, Write};

use crate::*;

pub static PRINT: StationType = StationType {
    id: "print",
    alt_id: None,
    inputs: 1,
    output: false,
    procedure: print_procedure,
    description: "",
};
fn print_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match &pallets[0] {
        Some(Pallet::Empty) => {}
        Some(Pallet::Bool(b)) => {
            if *b {
                print!("true");
            } else {
                print!("false");
            }
        }
        Some(Pallet::Char(c)) => {
            print!("{c}");
        }
        Some(Pallet::String(s)) => {
            print!("{s}");
        }
        Some(Pallet::Int(i)) => {
            print!("{i}");
        }
        Some(Pallet::Float(f)) => {
            print!("{f}");
        }
        None => return Err(String::from("Missing pallet in print")),
    }
    return Ok(None);
}

pub static PRINTLN: StationType = StationType {
    id: "println",
    alt_id: None,
    inputs: 1,
    output: false,
    procedure: println_procedure,
    description: "",
};
fn println_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    print_procedure(pallets)?;
    println!();
    return Ok(None);
}

pub static READLN: StationType = StationType {
    id: "readln",
    alt_id: None,
    inputs: 1,
    output: true,
    procedure: readln_procedure,
    description: "",
};
fn readln_procedure(_: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    let mut input = String::new();
    let _ = stdout().flush();
    match stdin().read_line(&mut input) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => {
            return Ok(Some(Pallet::String(if input.ends_with('\n') {
                input.strip_suffix('\n').unwrap().to_owned()
            } else {
                input
            })));
        }
    }
}
