use core::*;
use lazy_static::lazy_static;

#[cfg(test)]
mod tests;

lazy_static! {
    pub static ref NAMES: Vec<&'static StationType<'static>> =
        vec![&START, &EXIT, &JOINT, &ASSIGN, &PRINT, &PRINTLN];
}

/// Common procedure that returns nothign
fn none_procedure(_: &Vec<Pallet>) -> Result<Option<Pallet>, String> {
    return Ok(None);
}

static START: StationType = StationType {
    id: "start",
    inputs: 0,
    output: true,
    procedure: start_procedure,
};
fn start_procedure(_: &Vec<Pallet>) -> Result<Option<Pallet>, String> {
    return Ok(Some(Pallet::Empty));
}

static EXIT: StationType = StationType {
    id: "exit",
    inputs: 1,
    output: false,
    procedure: none_procedure,
};

static JOINT: StationType = StationType {
    id: "",
    inputs: 1,
    output: true,
    procedure: none_procedure,
};

static ASSIGN: StationType = StationType {
    id: "assign",
    inputs: 1,
    output: true,
    procedure: none_procedure,
};

static PRINT: StationType = StationType {
    id: "print",
    inputs: 1,
    output: false,
    procedure: print_procedure,
};
fn print_procedure(pallets: &Vec<Pallet>) -> Result<Option<Pallet>, String> {
    match pallets.iter().next().unwrap() {
        Pallet::Empty => {
            println!();
        }
        Pallet::Bool(b) => {
            if *b {
                println!("true");
            } else {
                println!("false");
            }
        }
        Pallet::Char(c) => {
            println!("{c}");
        }
        Pallet::String(s) => {
            println!("{s}");
        }
        Pallet::Int(i) => {
            println!("{i}");
        }
        Pallet::Float(f) => {
            println!("{f:.8}");
        }
    }
    return Ok(None);
}

static PRINTLN: StationType = StationType {
    id: "println",
    inputs: 1,
    output: false,
    procedure: println_procedure,
};
fn println_procedure(pallets: &Vec<Pallet>) -> Result<Option<Pallet>, String> {
    print_procedure(pallets)?;
    println!();
    return Ok(None);
}

/*
/// Search stdlib stations by id string
pub fn search_registry(id: &str) -> Option<Box<&'static dyn StationType>> {
    Some(Box::new(match id {
        "start" => &Start {},
        "println" => &Println {},
        _ => return None,
    }))
}


pub fn get_assign_station(value: Pallet) -> Option<Box<&'static dyn StationType>> {
    Some(Box::new(&Assign {}))
}

impl StationType for Types {
    const INPUTS: usize = 0;
    const OUTPUT: bool = true;
    fn procedure(&mut self, _: Vec<Pallet>) -> Result<Option<Pallet>, String> {
        return Ok(Some(Pallet::Empty));
    }
}

pub struct Assign {}
impl StationType for Assign {
    const INPUTS: usize = 1;
    const OUTPUT: bool = true;
    fn procedure(&mut self, _: Vec<Pallet>) -> Result<Option<Pallet>, String> {
        panic!("Shouldn't be called, assignment handled manually in the interpreter")
    }
}

pub struct Println;
impl StationType for Println {
    const INPUTS: usize = 1;
    const OUTPUT: bool = false;
    fn procedure(&mut self, pallets: Vec<Pallet>) -> Result<Option<Pallet>, String> {
        match &pallets[0] {
            Pallet::Empty => {
                println!();
            }
            Pallet::Bool(b) => {
                if *b {
                    println!("true");
                } else {
                    println!("false");
                }
            }
            Pallet::Char(c) => {
                println!("{c}");
            }
            Pallet::String(s) => {
                println!("{s}");
            }
            Pallet::Int(i) => {
                println!("{i}");
            }
            Pallet::Float(f) => {
                println!("{f:.8}");
            }
        }
        return Ok(None);
    }
}

pub struct Exit;
impl StationType for Exit {
    const INPUTS: usize = 1;
    const OUTPUT: bool = false;
    fn procedure(&mut self, _: Vec<Pallet>) -> Result<Option<Pallet>, String> {
        return Ok(None);
    }
}
 */
