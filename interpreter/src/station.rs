use crate::*;
use fs_core::*;
use stdlib;

/// Instance of a station
pub struct Station<'a> {
    /// Location of the station in source code
    pub loc: SourceLocation,
    /// Station functionality and type information
    pub logic: &'a dyn StationType,
    /// Modifiers duh
    pub modifiers: StationModifiers,
    /// Queues for each input bay
    pub in_bays: Vec<Option<Pallet>>,
    /// Map of each output bay connection in the form (station_index, in_bay_index)
    pub out_bays: Vec<(usize, usize)>,
}
impl Station<'_> {
    pub fn new(identifier: &str, loc: SourceLocation, modifiers: StationModifiers) -> Self {
        Self {
            loc,
            logic: &stdlib::Start,
            modifiers,
            in_bays: Vec::new(),
            out_bays: Vec::new(),
        }
    }
    pub fn new_in_bay(&mut self) {
        self.in_bays.push(None);
    }
}
/*impl std::fmt::Display for Station<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.t, self.loc)
    }
}*/

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
