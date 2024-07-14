use crate::*;
use core::*;

/// Instance of a station
#[derive(Debug)]
pub struct Station {
    /// Location of the station in source code
    pub loc: SourceLocation,
    /// Station functionality and type information
    pub logic: &'static StationType<'static>,
    /// Modifiers duh
    pub modifiers: StationModifiers,
    /// Queues for each input bay
    pub in_bays: Vec<Option<Pallet>>,
    /// Map of each output bay connection in the form (station_index, in_bay_index)
    pub out_bays: Vec<(usize, usize)>,
}
impl Station {
    pub fn new(
        identifier: &str,
        loc: SourceLocation,
        modifiers: StationModifiers,
        ns: &Namespace,
    ) -> Result<Self, Error> {
        for station_type in ns {
            if station_type.has_id(identifier) {
                return Ok(Self {
                    loc,
                    logic: station_type,
                    modifiers,
                    in_bays: Vec::new(),
                    out_bays: Vec::new(),
                });
            }
        }
        return Err(Error {
            t: ErrorType::IdentifierError,
            loc,
            msg: format!("Failed to find station type with identifier \"{identifier}\""),
        });
    }

    pub fn clear_in_bays(&mut self) {
        for bay in self.in_bays.iter_mut() {
            if bay.is_some() {
                *bay = None;
            }
        }
    }
}

/// Struct for holding the modifiers of an instance of a station
#[derive(Debug)]
pub struct StationModifiers {
    /// Reverse input precedence (false=cw, true=ccw)
    pub reverse: bool,
    /// Which direction the precedence starts with
    pub priority: Direction,
}
impl StationModifiers {
    /// Default modifiers for a station
    pub fn default() -> Self {
        Self {
            reverse: false,
            priority: Direction::NORTH,
        }
    }
    /// toggles the reverse direction modifier
    pub fn reverse(self) -> Self {
        Self {
            reverse: !self.reverse,
            ..self
        }
    }
    /// sets the direction with priority to a new value
    pub fn with_priority(self, new_priority: Direction) -> Self {
        Self {
            priority: new_priority,
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_station_clear_in_bays() {
        let mut station = Station::new(
            "joint",
            SourceLocation::none(),
            StationModifiers::default(),
            &stdlib::MANIFEST,
        )
        .unwrap();
        station.in_bays.push(Some(Pallet::Empty));
        station.in_bays.push(Some(Pallet::Int(3)));
        station.in_bays.push(Some(Pallet::Char('a')));
        station.clear_in_bays();
        assert!(station.in_bays[0].is_none());
        assert!(station.in_bays[1].is_none());
        assert!(station.in_bays[2].is_none());
    }

    #[test]
    fn test_station_modifiers() {
        assert!(matches!(
            StationModifiers::default(),
            StationModifiers {
                reverse: false,
                priority: Direction::NORTH
            }
        ));
        assert!(matches!(
            StationModifiers::default().reverse(),
            StationModifiers {
                reverse: true,
                priority: Direction::NORTH
            }
        ));
        assert!(matches!(
            StationModifiers::default().with_priority(Direction::SOUTH),
            StationModifiers {
                reverse: false,
                priority: Direction::SOUTH
            }
        ));
        assert!(matches!(
            StationModifiers::default()
                .reverse()
                .with_priority(Direction::EAST),
            StationModifiers {
                reverse: true,
                priority: Direction::EAST
            }
        ));
    }
}
