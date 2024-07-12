use fs_core::*;
use lazy_static::lazy_static;
use paste::paste;

#[cfg(test)]
mod tests;

lazy_static! {
    pub static ref NAMESPACE: Vec<&'static StationType<'static>> = vec![&Start, &Exit];
}

define_station!(Start {
    id: "start",
    inputs: 0,
    output: true,
    procedure: (pallets: Vec<Pallet>) -> Result<Option<Pallet>, String> {
        return Ok(Some(Pallet::Empty));
    }
});

define_station!(Exit {
    id: "exit",
    inputs: 1,
    output: false,
    procedure: (pallets: Vec<Pallet>) -> Result<Option<Pallet>, String> {
        return Ok(None);
    }
});

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
