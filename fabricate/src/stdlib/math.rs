use core::*;

pub static EQUALS: StationType = StationType {
    id: "eq",
    alt_id: Some("="),
    inputs: 2,
    output: true,
    procedure: equals_procedure,
};
fn equals_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    return Ok(Some(Pallet::Bool(pallets[0] == pallets[1])));
}

pub static NOT_EQUALS: StationType = StationType {
    id: "ne",
    alt_id: Some("!="),
    inputs: 2,
    output: true,
    procedure: not_equals_procedure,
};
fn not_equals_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    return Ok(Some(Pallet::Bool(pallets[0] != pallets[1])));
}

pub static ADD: StationType = StationType {
    id: "add",
    alt_id: Some("+"),
    inputs: 2,
    output: true,
    procedure: add_procedure,
};
fn add_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Int(num1)), Some(Pallet::Int(num2))) => {
            return Ok(Some(Pallet::Int(num1 + num2)));
        }
        (Some(Pallet::Float(num1)), Some(Pallet::Float(num2))) => {
            return Ok(Some(Pallet::Float(num1 + num2)));
        }
        (Some(Pallet::Char(char1)), Some(Pallet::Char(char2))) => {
            let mut s = char1.to_string();
            s.push(*char2);
            return Ok(Some(Pallet::String(s)));
        }
        (Some(Pallet::String(string)), Some(Pallet::Char(char))) => {
            let mut s = string.to_owned();
            s.push(*char);
            return Ok(Some(Pallet::String(s)));
        }
        (Some(Pallet::String(string1)), Some(Pallet::String(string2))) => {
            return Ok(Some(Pallet::String(string1.to_owned() + string2.as_str())));
        }
        _ => {
            return Err(format!(
                "Unexpected pallets received: ({}, {})\n",
                match &pallets[0] {
                    Some(pallet) => format!("{pallet}"),
                    None => String::from("None"),
                },
                match &pallets[1] {
                    Some(pallet) => format!("{pallet}"),
                    None => String::from("None"),
                },
            ));
        }
    }
}

/*
pub static SUBTRACT: StationType = StationType {
    id: "subtract",
    alt_id: Some("-"),
    inputs: 2,
    output: true,
    procedure: subtract_procedure,
};
fn subtract_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match pallets[0] {
        None => return Err(String::from("No pallets provided")),
        Some(Pallet::Int(num1)) => {
            if let Some(Pallet::Int(num2)) = pallets[1] {
                return Ok(Some(Pallet::Int(num1 - num2)));
            }
            return Err(String::from("Mismatching types, expected integer pallet"));
        }
        Some(Pallet::Float(num1)) => {
            if let Some(Pallet::Float(num2)) = pallets[1] {
                return Ok(Some(Pallet::Float(num1 - num2)));
            }
            return Err(String::from("Mismatching types, expected float pallet"));
        }
        _ => {
            return Err(format!(
                "Expected integer or float pallet, received {:?}",
                pallets[0]
            ));
        }
    }
}
 */
