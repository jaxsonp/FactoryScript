use core::*;

pub static ADD: StationType = StationType {
    id: "add",
    alt_id: Some("+"),
    inputs: 2,
    output: true,
    procedure: add_procedure,
};
fn add_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match &pallets[0] {
        Some(Pallet::Int(num1)) => {
            if let Some(Pallet::Int(num2)) = pallets[1] {
                return Ok(Some(Pallet::Int(num1 + num2)));
            }
            return Err(String::from("Mismatching types, expected integer pallet"));
        }
        Some(Pallet::Float(num1)) => {
            if let Some(Pallet::Float(num2)) = pallets[1] {
                return Ok(Some(Pallet::Float(num1 + num2)));
            }
            return Err(String::from("Mismatching types, expected float pallet"));
        }
        Some(Pallet::Char(char1)) => {
            if let Some(Pallet::Char(char2)) = pallets[1] {
                let s = char1.to_string() + &char2.to_string();
                return Ok(Some(Pallet::String(s)));
            }
            return Err(String::from("Mismatching types, expected char pallet"));
        }
        Some(Pallet::String(str1)) => {
            if let Some(Pallet::Char(char)) = pallets[1] {
                return Ok(Some(Pallet::String(str1.to_owned() + &char.to_string())));
            } else if let Some(Pallet::String(str2)) = &pallets[1] {
                return Ok(Some(Pallet::String(str1.to_owned() + &str2)));
            }
            return Err(String::from(
                "Mismatching types, expected string or char pallet",
            ));
        }
        Some(Pallet::Bool(b1)) => {
            if let Some(Pallet::Bool(b2)) = pallets[1] {
                return Ok(Some(Pallet::Bool(*b1 || b2)));
            }
            return Err(String::from("Mismatching types, expected boolean pallet"));
        }
        Some(Pallet::Empty) => {
            return Err(String::from("Cannot perform addition on empty pallets"))
        }
        None => return Err(String::from("No pallets provided")),
    }
}
