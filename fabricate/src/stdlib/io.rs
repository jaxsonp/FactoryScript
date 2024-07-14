use core::*;

pub static PRINT: StationType = StationType {
    id: "print",
    alt_id: None,
    inputs: 1,
    output: false,
    procedure: print_procedure,
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
};
fn println_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    print_procedure(pallets)?;
    println!();
    return Ok(None);
}
