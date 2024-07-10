use regex::Regex;

use crate::*;

mod probe;
use probe::Bay;

/// Preprocesses a source string, validating the syntax and grammar
///
/// Returns a tuple containing a vector of stations and the index of the start
/// station, or an `Error` if unsuccessful
pub fn process(lines: &Vec<&str>) -> Result<(Vec<Station>, usize), Error> {
    // discovery
    debug!(3, "Discovering stations");
    let mut stations = discover_stations(lines)?;
    debug!(3, "Found {} stations", stations.len());

    // searching for start index
    let mut start_index = 0;
    for i in 0..stations.len() {
        if matches!(stations[i].t, StationType::START) {
            start_index = i;
        }
    }

    // generating 2d vector layout of source code
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.chars().collect());
    }

    // probing connected bays
    debug!(3, "Parsing station bays");
    for i in 0..stations.len() {
        debug!(3, " - #{i}'s bays",);
        for neighbor in get_neighbors(&map, &mut stations[i]) {
            // checking if each neighbor is a bay and what type
            if let Some(bay) = probe::evaluate_bay(&map, neighbor)? {
                match bay {
                    Bay::In => {
                        debug!(3, "    <");
                    }
                    Bay::Out(line, col) => {
                        // getting station at dest position
                        let mut dest_station = 0;
                        let mut dest_is_station = false;
                        for j in 0..stations.len() {
                            if line == stations[j].loc.line
                                && col >= stations[j].loc.col
                                && col < stations[j].loc.col + stations[j].loc.len
                            {
                                dest_station = j;
                                dest_is_station = true;
                                break;
                            }
                        }
                        if !dest_is_station {
                            return Err(Error {
                                t: ErrorType::SyntaxError(SourceLocation { line, col, len: 1 }),
                                msg: String::from("Dangling conveyor belt"),
                            });
                        }
                        debug!(3, "    > #{}", dest_station);
                    }
                }
            }
        }
    }

    debug!(2, "Finished preprocessing");
    Ok((stations, start_index))
}

/// Finds all stations in the source code and parses their type and modifiers
fn discover_stations(lines: &Vec<&str>) -> Result<Vec<Station>, Error> {
    // regex for matching stations
    let station_re =
        Regex::new(r"(\[[a-zA-Z0-9- ]*(?::[!NSEW]*)?\])|(\{[a-zA-Z0-9- ]*(?::[!NSEW]*)?\})")
            .unwrap();

    let mut stations: Vec<Station> = Vec::new();
    let mut start_found = false;
    for i in 0..lines.len() {
        for m in station_re.find_iter(lines[i]) {
            // string parsing
            let text = m.as_str();
            let stripped = &text[1..text.len() - 1];
            let mut split = stripped.split(':');
            let name = split.next().unwrap();
            let loc = SourceLocation {
                line: i,
                col: get_char_index_from_byte_offset(m.start(), lines[i]),
                len: m.len(),
            };
            debug!(3, " - #{} @ {} {}", stations.len(), loc, text);

            if name == "start" {
                if start_found {
                    return Err(Error {
                        t: ErrorType::SyntaxError(loc),
                        msg: String::from("Defined multiple start stations"),
                    });
                }
                start_found = true;
            }

            // parsing station modifiers
            let mut modifiers = StationModifiers::default();
            if let Some(mod_string) = split.next() {
                if mod_string.contains('!') {
                    modifiers.reverse = true;
                    debug!(4, "   - reverse modifier");
                }
                let mut direction_specified = false;
                // closure that checks if a direction has already been specified
                let mut check_multiple_directions = || -> Result<(), Error> {
                    if direction_specified {
                        return Err(Error {
                            t: ErrorType::SyntaxError(loc),
                            msg: String::from("Specified multiple direction priority modifiers"),
                        });
                    }
                    direction_specified = true;
                    Ok(())
                };
                if mod_string.contains('N') {
                    check_multiple_directions()?;
                    modifiers.priority = Direction::NORTH;
                    debug!(4, "   - north modifier");
                }
                if mod_string.contains('E') {
                    check_multiple_directions()?;
                    modifiers.priority = Direction::EAST;
                    debug!(4, "   - east modifier");
                }
                if mod_string.contains('S') {
                    check_multiple_directions()?;
                    modifiers.priority = Direction::SOUTH;
                    debug!(4, "   - south modifier");
                }
                if mod_string.contains('W') {
                    check_multiple_directions()?;
                    modifiers.priority = Direction::WEST;
                    debug!(4, "   - west modifier");
                }
            }

            stations.push(Station {
                loc,
                t: if text.starts_with('{') {
                    StationType::ASSIGN(String::from(stripped))
                } else {
                    StationType::from_str(name)
                },
                modifiers,
                in_bays: Vec::new(),
                out_bays: Vec::new(),
            });
        }
    }
    if !start_found {
        return Err(Error {
            t: ErrorType::SyntaxError(SourceLocation {
                line: 0,
                col: 0,
                len: 0,
            }),
            msg: String::from("Missing a start station"),
        });
    }
    return Ok(stations);
}

/// helper function to get the index of a unicode character from the byte offset
/// in a string slice
///
/// I need this cus the regex searching above only returns a byte offset but I need
/// the station's positions in terms of complete characters
fn get_char_index_from_byte_offset(byte_offset: usize, s: &str) -> usize {
    let s = String::from(s);
    let mut char_index = 0;
    for (pos, _) in s.char_indices() {
        if byte_offset <= pos {
            return char_index;
        }
        char_index += 1;
    }
    return char_index;
}

/// Returns a vector of all valid bay positions around a station
fn get_neighbors(map: &Vec<Vec<char>>, station: &Station) -> Vec<(usize, usize, Direction)> {
    let mut neighbors: Vec<(usize, usize, Direction)> = Vec::new();

    let mut northern_neighbors: Vec<(usize, usize, Direction)> = Vec::new();
    if station.loc.line > 0 {
        for i in 0..station.loc.len {
            if i < map[station.loc.line - 1].len() {
                northern_neighbors.push((
                    station.loc.line - 1,
                    station.loc.col + i,
                    Direction::NORTH,
                ));
            }
        }
    }
    let mut eastern_neighbors: Vec<(usize, usize, Direction)> = Vec::new();
    if station.loc.col + station.loc.len < map[station.loc.line].len() {
        eastern_neighbors.push((
            station.loc.line,
            station.loc.col + station.loc.len,
            Direction::EAST,
        ));
    }
    let mut southern_neighbors: Vec<(usize, usize, Direction)> = Vec::new();
    if station.loc.line < (map.len() - 1) {
        for i in (0..station.loc.len).rev() {
            if i < map[station.loc.line + 1].len() {
                southern_neighbors.push((
                    station.loc.line + 1,
                    station.loc.col + i,
                    Direction::SOUTH,
                ));
            }
        }
    }
    let mut western_neighbors: Vec<(usize, usize, Direction)> = Vec::new();
    if station.loc.col > 0 {
        western_neighbors.push((station.loc.line, station.loc.col - 1, Direction::WEST));
    }

    if !station.modifiers.reverse {
        // clockwise
        match station.modifiers.priority {
            Direction::NORTH => {
                neighbors.extend(northern_neighbors);
                neighbors.extend(eastern_neighbors);
                neighbors.extend(southern_neighbors);
                neighbors.extend(western_neighbors);
            }
            Direction::EAST => {
                neighbors.extend(eastern_neighbors);
                neighbors.extend(southern_neighbors);
                neighbors.extend(western_neighbors);
                neighbors.extend(northern_neighbors);
            }
            Direction::SOUTH => {
                neighbors.extend(southern_neighbors);
                neighbors.extend(western_neighbors);
                neighbors.extend(northern_neighbors);
                neighbors.extend(eastern_neighbors);
            }
            Direction::WEST => {
                neighbors.extend(western_neighbors);
                neighbors.extend(northern_neighbors);
                neighbors.extend(eastern_neighbors);
                neighbors.extend(southern_neighbors);
            }
        }
    } else {
        // counter clockwise
        match station.modifiers.priority {
            Direction::NORTH => {
                neighbors.extend(northern_neighbors.iter().rev());
                neighbors.extend(western_neighbors);
                neighbors.extend(southern_neighbors.iter().rev());
                neighbors.extend(eastern_neighbors);
            }
            Direction::EAST => {
                neighbors.extend(eastern_neighbors);
                neighbors.extend(northern_neighbors.iter().rev());
                neighbors.extend(western_neighbors);
                neighbors.extend(southern_neighbors.iter().rev());
            }
            Direction::SOUTH => {
                neighbors.extend(southern_neighbors.iter().rev());
                neighbors.extend(eastern_neighbors);
                neighbors.extend(northern_neighbors.iter().rev());
                neighbors.extend(western_neighbors);
            }
            Direction::WEST => {
                neighbors.extend(western_neighbors);
                neighbors.extend(southern_neighbors.iter().rev());
                neighbors.extend(eastern_neighbors);
                neighbors.extend(northern_neighbors.iter().rev());
            }
        }
    }
    return neighbors;
}
