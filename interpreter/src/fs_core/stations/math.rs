use super::list_pallets;
use crate::*;

pub static EQUALS: StationType = StationType {
    id: "eq",
    alt_id: Some("="),
    inputs: 2,
    output: true,
    procedure: equals_procedure,
    description: "",
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
    description: "",
};
fn not_equals_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    return Ok(Some(Pallet::Bool(pallets[0] != pallets[1])));
}

pub static GREATER_THAN: StationType = StationType {
    id: "gt",
    alt_id: Some(">"),
    inputs: 2,
    output: true,
    procedure: greater_than_procedure,
    description: "",
};
fn greater_than_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Int(num1)), Some(Pallet::Int(num2))) => {
            return Ok(Some(Pallet::Bool(num1 > num2)));
        }
        (Some(Pallet::Float(num1)), Some(Pallet::Float(num2))) => {
            return Ok(Some(Pallet::Bool(num1 > num2)));
        }
        (Some(Pallet::Bool(bool1)), Some(Pallet::Bool(bool2))) => {
            return Ok(Some(Pallet::Bool(bool1 > bool2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical or boolean pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static LESS_THAN: StationType = StationType {
    id: "lt",
    alt_id: Some("<"),
    inputs: 2,
    output: true,
    procedure: less_than_procedure,
    description: "",
};
fn less_than_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Int(num1)), Some(Pallet::Int(num2))) => {
            return Ok(Some(Pallet::Bool(num1 < num2)));
        }
        (Some(Pallet::Float(num1)), Some(Pallet::Float(num2))) => {
            return Ok(Some(Pallet::Bool(num1 < num2)));
        }
        (Some(Pallet::Bool(bool1)), Some(Pallet::Bool(bool2))) => {
            return Ok(Some(Pallet::Bool(bool1 < bool2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical or boolean pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static GREATER_THAN_EQUAL: StationType = StationType {
    id: "gte",
    alt_id: Some(">="),
    inputs: 2,
    output: true,
    procedure: greater_than_equal_procedure,
    description: "",
};
fn greater_than_equal_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Int(num1)), Some(Pallet::Int(num2))) => {
            return Ok(Some(Pallet::Bool(num1 >= num2)));
        }
        (Some(Pallet::Float(num1)), Some(Pallet::Float(num2))) => {
            return Ok(Some(Pallet::Bool(num1 >= num2)));
        }
        (Some(Pallet::Bool(bool1)), Some(Pallet::Bool(bool2))) => {
            return Ok(Some(Pallet::Bool(bool1 >= bool2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical or boolean pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static LESS_THAN_EQUAL: StationType = StationType {
    id: "lte",
    alt_id: Some("<="),
    inputs: 2,
    output: true,
    procedure: less_than_equal_procedure,
    description: "",
};
fn less_than_equal_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Int(num1)), Some(Pallet::Int(num2))) => {
            return Ok(Some(Pallet::Bool(num1 <= num2)));
        }
        (Some(Pallet::Float(num1)), Some(Pallet::Float(num2))) => {
            return Ok(Some(Pallet::Bool(num1 <= num2)));
        }
        (Some(Pallet::Bool(bool1)), Some(Pallet::Bool(bool2))) => {
            return Ok(Some(Pallet::Bool(bool1 <= bool2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical or boolean pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static ADD: StationType = StationType {
    id: "add",
    alt_id: Some("+"),
    inputs: 2,
    output: true,
    procedure: add_procedure,
    description: "",
};
fn add_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Int(num1)), Some(Pallet::Int(num2))) => {
            return Ok(Some(Pallet::Int(num1 + num2)));
        }
        (Some(Pallet::Float(num1)), Some(Pallet::Float(num2))) => {
            return Ok(Some(Pallet::Float(num1 + num2)));
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
                "Unexpected pallet types received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static SUBTRACT: StationType = StationType {
    id: "sub",
    alt_id: Some("-"),
    inputs: 2,
    output: true,
    procedure: subtract_procedure,
    description: "",
};
fn subtract_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Int(num1)), Some(Pallet::Int(num2))) => {
            return Ok(Some(Pallet::Int(num1 - num2)));
        }
        (Some(Pallet::Float(num1)), Some(Pallet::Float(num2))) => {
            return Ok(Some(Pallet::Float(num1 - num2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static MULTIPLY: StationType = StationType {
    id: "mult",
    alt_id: Some("*"),
    inputs: 2,
    output: true,
    procedure: multiply_procedure,
    description: "",
};
fn multiply_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Int(num1)), Some(Pallet::Int(num2))) => {
            return Ok(Some(Pallet::Int(num1 * num2)));
        }
        (Some(Pallet::Float(num1)), Some(Pallet::Float(num2))) => {
            return Ok(Some(Pallet::Float(num1 * num2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static DIVIDE: StationType = StationType {
    id: "div",
    alt_id: Some("/"),
    inputs: 2,
    output: true,
    procedure: divide_procedure,
    description: "",
};
fn divide_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Int(num1)), Some(Pallet::Int(num2))) => {
            if *num2 == 0 {
                return Err(String::from("Attempted divide by zero"));
            }
            return Ok(Some(Pallet::Int(num1 / num2)));
        }
        (Some(Pallet::Float(num1)), Some(Pallet::Float(num2))) => {
            if *num2 == 0.0 {
                return Err(String::from("Attempted divide by zero"));
            }
            return Ok(Some(Pallet::Float(num1 / num2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static MODULO: StationType = StationType {
    id: "mod",
    alt_id: Some("%"),
    inputs: 2,
    output: true,
    procedure: modulo_procedure,
    description: "",
};
fn modulo_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Int(num1)), Some(Pallet::Int(num2))) => {
            if *num2 == 0 {
                return Err(String::from("Attempted divide by zero"));
            }
            return Ok(Some(Pallet::Int(num1 % num2)));
        }
        (Some(Pallet::Float(num1)), Some(Pallet::Float(num2))) => {
            if *num2 == 0.0 {
                return Err(String::from("Attempted divide by zero"));
            }
            return Ok(Some(Pallet::Float(num1 % num2)));
        }
        _ => {
            return Err(format!(
                "Expected numerical pallets, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static INCREMENT: StationType = StationType {
    id: "inc",
    alt_id: Some("++"),
    inputs: 1,
    output: true,
    procedure: increment_procedure,
    description: "",
};
fn increment_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match &pallets[0] {
        Some(Pallet::Int(num)) => {
            return Ok(Some(Pallet::Int(num + 1)));
        }
        Some(Pallet::Float(num)) => {
            return Ok(Some(Pallet::Float(num + 1.0)));
        }
        _ => {
            return Err(format!(
                "Expected one numerical pallet, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static DECREMENT: StationType = StationType {
    id: "dec",
    alt_id: Some("--"),
    inputs: 1,
    output: true,
    procedure: decrement_procedure,
    description: "",
};
fn decrement_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match &pallets[0] {
        Some(Pallet::Int(num)) => {
            return Ok(Some(Pallet::Int(num - 1)));
        }
        Some(Pallet::Float(num)) => {
            return Ok(Some(Pallet::Float(num - 1.0)));
        }
        _ => {
            return Err(format!(
                "Expected one numerical pallet, received: {}\n",
                list_pallets(pallets)
            ));
        }
    }
}

pub static AND: StationType = StationType {
    id: "and",
    alt_id: None,
    inputs: 2,
    output: true,
    procedure: and_procedure,
    description: "",
};
fn and_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Bool(b1)), Some(Pallet::Bool(b2))) => Ok(Some(Pallet::Bool(*b1 && *b2))),
        _ => {
            return Err(format!(
                "Expected two boolean pallets, received {}\n",
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
    description: "",
};
fn or_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match (&pallets[0], &pallets[1]) {
        (Some(Pallet::Bool(b1)), Some(Pallet::Bool(b2))) => Ok(Some(Pallet::Bool(*b1 || *b2))),
        _ => {
            return Err(format!(
                "Expected two boolean pallets, received {}\n",
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
    description: "",
};
fn not_procedure(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String> {
    match &pallets[0] {
        Some(Pallet::Bool(b1)) => Ok(Some(Pallet::Bool(!(*b1)))),
        _ => {
            return Err(format!(
                "Expected two boolean pallets, received {}\n",
                list_pallets(pallets)
            ));
        }
    }
}
