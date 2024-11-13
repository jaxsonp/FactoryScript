use super::*;

pub static START: StationType = StationType {
    id: "start",
    alt_id: None,
    inputs: 0,
    output: true,
    procedure: start_procedure,
    description: "",
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
    description: "",
};

pub static JOINT: StationType = StationType {
    id: "joint",
    alt_id: Some(""),
    inputs: 1,
    output: true,
    procedure: none_procedure,
    description: "",
};

pub static ASSIGN: StationType = StationType {
    id: "assign",
    alt_id: None,
    inputs: 1,
    output: true,
    procedure: none_procedure,
    description: "",
};

pub static GATE: StationType = StationType {
    id: "gate",
    alt_id: None,
    inputs: 2,
    output: true,
    procedure: gate_procedure,
    description: "",
};
fn gate_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
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

pub static FILTER: StationType = StationType {
    id: "filter",
    alt_id: Some("X"),
    inputs: 1,
    output: true,
    procedure: filter_procedure,
    description: "",
};
fn filter_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match &pallets[0] {
        Some(Pallet::Bool(false)) => Ok(None),
        Some(p) => Ok(Some(p.clone())),
        _ => {
            return Err(format!(
                "Expected pallet, received {}\n",
                list_pallets(pallets)
            ));
        }
    }
}
