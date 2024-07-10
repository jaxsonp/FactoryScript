use std::collections::VecDeque;

use crate::*;

/// Instance of a station
pub struct Station {
    /// Location of the station in source code
    pub loc: SourceLocation,
    /// Station type
    pub t: StationType,
    /// Modifiers duh
    pub modifiers: StationModifiers,
    /// Queues for each input bay
    pub in_bays: Vec<VecDeque<Pallet>>,
    /// Map of each output bay connection in the form (station_index, in_bay_index)
    pub out_bays: Vec<(usize, usize)>,
}
impl std::fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.t, self.loc)
    }
}

/// Built in station types
pub enum StationType {
    START,
    EXIT,
    PRINT,
    PRINTLN,
    ASSIGN(String),
    JOIN,
}
impl StationType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "start" => Self::START,
            "exit" => Self::EXIT,
            "print" => Self::PRINT,
            "println" => Self::PRINTLN,
            "" => Self::JOIN,
            _ => {
                todo!("{}", s)
            }
        }
    }
}
impl std::fmt::Display for StationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::START => String::from("start"),
            Self::EXIT => String::from("exit"),
            Self::PRINT => String::from("print"),
            Self::PRINTLN => String::from("println"),
            Self::ASSIGN(s) => {
                format!("assign [{s}])")
            }
            Self::JOIN => String::from("join"),
        };
        write!(f, "{s}")
    }
}

pub struct StationModifiers {
    /// Reverse input precedence (false=cw, true=ccw)
    pub reverse: bool,
    /// Which direction the precedence starts with
    pub priority: Direction,
}
impl StationModifiers {
    pub fn default() -> Self {
        Self {
            reverse: false,
            priority: Direction::NORTH,
        }
    }
    pub fn reverse(self) -> Self {
        Self {
            reverse: !self.reverse,
            ..self
        }
    }
    pub fn with_priority(self, new_priority: Direction) -> Self {
        Self {
            priority: new_priority,
            ..self
        }
    }
}
