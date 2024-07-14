use core::*;
use lazy_static::lazy_static;

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
        &io::PRINT,
        &io::PRINTLN,
        &math::ADD
    ];
}

/// Common procedure that returns nothign
pub fn none_procedure(_: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    return Ok(None);
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
