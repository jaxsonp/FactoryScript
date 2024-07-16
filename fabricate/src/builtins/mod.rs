use core::*;
use lazy_static::lazy_static;
use std::ops::Not;

#[cfg(test)]
mod tests;

mod io;
mod math;

lazy_static! {
    pub static ref MANIFEST: Vec<&'static StationType<'static>> = vec![
        &START,
        &EXIT,
        &JOINT,
        &ASSIGN,
        &AND,
        &NOT,
        &OR,
        &io::PRINT,
        &io::PRINTLN,
        &math::ADD,
        &math::SUBTRACT,
        &math::MULTIPLY,
        &math::DIVIDE,
        &math::MODULO,
        &math::INCREMENT,
        &math::DECREMENT,
        &math::EQUALS,
        &math::NOT_EQUALS,
        &math::GREATER_THAN,
        &math::LESS_THAN,
        &math::GREATER_THAN_EQUAL,
        &math::LESS_THAN_EQUAL,
    ];
}

/// Common procedure that returns nothign
pub fn none_procedure(_: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    return Ok(None);
}

/// helper function to generate a string listing pallets, used for error messages
fn list_pallets(pallets: &Vec<Option<Pallet>>) -> String {
    let mut output = String::from("(");
    for i in 0..pallets.len() {
        output.push_str(
            match &pallets[0] {
                Some(pallet) => format!("{pallet}"),
                None => String::from("None"),
            }
            .as_str(),
        );
        if i != pallets.len() - 1 {
            output.push_str(", ")
        }
    }
    output.push(')');
    return output;
}

pub static START: StationType = StationType {
    id: "start",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: start_procedure,
};
fn start_procedure(_: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    return Ok(Some(Pallet::Empty));
}

pub static EXIT: StationType = StationType {
    id: "exit",
    alt_id: None,
    inputs: 1,
    output: false,
    procedure: none_procedure,
};

pub static JOINT: StationType = StationType {
    id: "joint",
    alt_id: Some(""),
    inputs: 1,
    output: true,
    procedure: none_procedure,
};

pub static ASSIGN: StationType = StationType {
    id: "assign",
    alt_id: None,
    inputs: 1,
    output: true,
    procedure: none_procedure,
};

pub static AND: StationType = StationType {
    id: "and",
    alt_id: None,
    inputs: 2,
    output: true,
    procedure: and_procedure,
};
fn and_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Bool(b)), Some(pallet)) => Ok(if *b { Some(pallet.clone()) } else { None }),
        (Some(pallet), Some(Pallet::Bool(b))) => Ok(if *b { Some(pallet.clone()) } else { None }),
        _ => {
            return Err(format!(
                "Expected at least one boolean pallet, received {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static OR: StationType = StationType {
    id: "or",
    alt_id: None,
    inputs: 2,
    output: true,
    procedure: or_procedure,
};
fn or_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Bool(b1)), Some(Pallet::Bool(b2))) => Ok(Some(Pallet::Bool(*b1 || *b2))),
        _ => {
            return Err(format!(
                "Expected at least one boolean pallet, received {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static NOT: StationType = StationType {
    id: "not",
    alt_id: Some("!"),
    inputs: 1,
    output: true,
    procedure: not_procedure,
};
fn not_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match &pallets[0] {
        Some(Pallet::Bool(b1)) => Ok(Some(Pallet::Bool(b1.not()))),
        _ => {
            return Err(format!(
                "Expected at least one boolean pallet, received {}\n",
                list_pallets(pallets)
            ));
        }
    }
}
