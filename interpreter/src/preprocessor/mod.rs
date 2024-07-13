use core::Pallet;
use regex::Regex;
use std::collections::HashMap;

pub mod probe;
#[cfg(test)]
mod tests;

use crate::*;

/// Preprocesses a source string, validating the syntax and grammar
///
/// Returns a tuple containing a vector of stations and the index of the start
/// station, or an `Error` if unsuccessful
pub fn process<'a>(
    lines: &Vec<&str>,
    ns: &Namespace,
) -> Result<(Vec<Station>, usize, HashMap<usize, Pallet>), Error> {
    // discovery
    debug!(3, "Discovering stations");
    let (mut stations, start_index, assign_table) = discover_stations(lines, ns)?;
    debug!(3, "Found {} stations", stations.len());

    // generating 2d vector layout of source code
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.chars().collect());
    }

    // probing connected bays
    debug!(3, "Parsing station bays");
    for i in 0..stations.len() {
        debug!(3, " - #{i}'s inputs:",);
        for neighbor in get_neighbors(&map, &mut stations[i]) {
            // checking if each neighbor is an input bay and finding the origin of the conveyor belt
            if let Some(origin_pos) = probe::evaluate_bay(&map, neighbor)? {
                let mut origin_station: Option<usize> = None;
                for j in 0..stations.len() {
                    if origin_pos.0 == stations[j].loc.line
                        && origin_pos.1 >= stations[j].loc.col
                        && origin_pos.1 < stations[j].loc.col + stations[j].loc.len
                    {
                        origin_station = Some(j);
                        break;
                    }
                }
                if origin_station.is_none() {
                    return Err(Error {
                        t: ErrorType::SyntaxError,
                        loc: SourceLocation {
                            line: origin_pos.0,
                            col: origin_pos.1,
                            len: 1,
                        },
                        msg: String::from("Dangling conveyor belt, expected station"),
                    });
                }
                let origin_station = origin_station.unwrap();
                let in_bay = stations[i].in_bays.len();
                stations[origin_station].out_bays.push((i, in_bay));
                stations[i].new_in_bay();
                debug!(3, "    - conveyor belt from #{origin_station}");
            }
        }
    }

    debug!(2, "Finished preprocessing");
    Ok((stations, start_index, assign_table))
}

/// Finds all stations in the source code, parses their type and modifiers, returns
/// a vector of all stations, the start station's index, and the assign table
/// (a hashmap containing the values of every assign station)
fn discover_stations(
    lines: &Vec<&str>,
    ns: &Namespace,
) -> Result<(Vec<Station>, usize, HashMap<usize, Pallet>), Error> {
    // regex for matching stations
    let station_re =
        //Regex::new(r"(\[[a-zA-Z0-9- ]*(?::[!NSEW]*)?\])|(\{[a-zA-Z0-9- ]*(?::[!NSEW]*)?\})")(\[.*(?::[!NSEW]*)?\])|(\{.*(?::[!NSEW]*)?\})
        Regex::new(r"(\[\s*[^\[\]\{\}]*(?::[!NSEW]*)?\s*\])|(\{[^\[\]\{\}]*\})")
            .unwrap();

    let mut stations: Vec<Station> = Vec::new();
    let mut assign_table: HashMap<usize, Pallet> = HashMap::new();
    let mut start_found = false;
    let mut start_index = 0;
    for i in 0..lines.len() {
        for m in station_re.find_iter(lines[i]) {
            let loc = SourceLocation {
                line: i,
                col: get_char_index_from_byte_offset(m.start(), lines[i]),
                len: m.len(),
            };

            // string parsing
            let text = m.as_str();
            let stripped = &text[1..text.len() - 1];
            if text.starts_with('{') {
                // assignment station
                // parsing type
                let value = if stripped == "true" {
                    Pallet::Bool(true)
                } else if stripped == "false" {
                    Pallet::Bool(false)
                } else if stripped.starts_with('\"') && stripped.ends_with('\"') {
                    Pallet::String(String::from(&stripped[1..stripped.len() - 1]))
                } else if stripped.starts_with('\'') && stripped.ends_with('\'') {
                    let chars: Vec<char> = stripped.chars().collect();
                    if chars.len() != 3 {
                        return Err(Error {
                            t: ErrorType::SyntaxError,
                            loc,
                            msg: String::from("Invalid character literal"),
                        });
                    }
                    Pallet::Char(chars[1])
                } else if stripped.ends_with('f') || is_float(stripped) {
                    let s = if stripped.ends_with('f') {
                        &stripped[..stripped.len() - 1]
                    } else {
                        stripped
                    };
                    let num = match s.parse::<f32>() {
                        Ok(num) => num,
                        Err(e) => {
                            return Err(Error {
                                t: ErrorType::SyntaxError,
                                loc,
                                msg: format!("Failed to parse float: {e}"),
                            });
                        }
                    };
                    println!("{num}");
                    Pallet::Float(num)
                } else if is_int(stripped) {
                    println!("int");
                    let s = stripped.replace('_', "");
                    let num = match s.parse::<i32>() {
                        Ok(num) => num,
                        Err(e) => {
                            return Err(Error {
                                t: ErrorType::SyntaxError,
                                loc,
                                msg: format!("Failed to parse integer: {e}"),
                            });
                        }
                    };
                    Pallet::Int(num)
                } else {
                    return Err(Error {
                        t: ErrorType::SyntaxError,
                        loc,
                        msg: format!("Unable to infer assignment type of \"{text}\""),
                    });
                };
                debug!(
                    3,
                    " - #{} @ {} {} contains {}",
                    stations.len(),
                    loc,
                    text,
                    value
                );
                assign_table.insert(stations.len(), value);
                stations.push(Station::new(
                    "assign",
                    loc,
                    StationModifiers::default(),
                    ns,
                )?);
                continue;
            }
            let split: Vec<&str> = stripped.split(':').collect();
            if split.len() > 2 {
                return Err(Error {
                    t: ErrorType::SyntaxError,
                    loc,
                    msg: String::from(
                        "Invalid station, modifiers must be of the form \"[<NAME>:<FLAGS>]\"",
                    ),
                });
            }
            let identifier = split[0];
            if identifier == "start" {
                if start_found {
                    return Err(Error {
                        t: ErrorType::SyntaxError,
                        loc,
                        msg: String::from("Factory must only define one start station"),
                    });
                }
                start_index = i;
                start_found = true;
            }
            for c in identifier.chars() {
                if identifier != "" && !c.is_ascii_alphanumeric() && c != '-' && c != ' ' {
                    return Err(Error { t: ErrorType::SyntaxError, loc, msg: String::from("Station identifiers must only contain a-z, A-Z, 0-9, dashes, and spaces.") });
                }
            }

            // parsing station modifiers
            let mut modifiers = StationModifiers::default();
            if split.len() == 2 {
                let mod_string = split[1];
                if mod_string.contains('!') {
                    modifiers.reverse = true;
                    debug!(4, "   - reverse modifier");
                }
                let mut direction_specified = false;
                // closure that checks if a direction modifier has already been specified
                let mut check_multiple_directions = || -> Result<(), Error> {
                    if direction_specified {
                        return Err(Error {
                            t: ErrorType::SyntaxError,
                            loc,
                            msg: String::from(
                                "Each station must contain only one direction priority modifier",
                            ),
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

            debug!(3, " - #{} @ {} {}", stations.len(), loc, text);
            stations.push(Station::new(identifier, loc, modifiers, ns)?);
        }
    }
    if !start_found {
        return Err(Error {
            t: ErrorType::SyntaxError,
            loc: SourceLocation {
                line: 0,
                col: 0,
                len: 0,
            },
            msg: String::from("Unable to locate start station"),
        });
    }
    return Ok((stations, start_index, assign_table));
}

/// helper function to check if a string can be parsed into an integer
///
/// # Example
/// ```
/// use interpreter::preprocessor::is_int;
/// assert_eq!(is_int("1234"), true);
/// assert_eq!(is_int("1_234_456"), true);
/// assert_eq!(is_int("12.34"), false);
/// assert_eq!(is_int("12a4"), false);
/// ```
#[inline]
pub fn is_int(s: &str) -> bool {
    for c in s.chars() {
        if !(c.is_ascii_digit() || c == '_') {
            return false;
        }
    }
    return true;
}

/// helper function to check if a string can be parsed into a float
///
/// # Example
/// ```
/// use interpreter::preprocessor::is_float;
/// assert_eq!(is_float("12.34"), true);
/// assert_eq!(is_float("1234"), true);
/// assert_eq!(is_float("1_234_456"), true);
/// assert_eq!(is_float("12a4"), false);
/// assert_eq!(is_float("12.3.4"), false);
/// ```
#[inline]
pub fn is_float(s: &str) -> bool {
    let mut found_decimal = false;
    for c in s.chars() {
        if c == '.' {
            if found_decimal {
                return false;
            }
            found_decimal = true;
        } else if !c.is_ascii_digit() && c != '_' {
            return false;
        }
    }
    return true;
}

/// helper function to get the index of a unicode character from the byte offset
/// in a string slice
///
/// I need this cus the regex searching above only returns a byte offset but I need
/// the station's positions in terms of complete characters
///
/// # Example
/// ```
/// use interpreter::preprocessor::get_char_index_from_byte_offset;
/// let s = "😼abcd";
/// assert_eq!(get_char_index_from_byte_offset(4, s), 1);
/// assert_eq!(get_char_index_from_byte_offset(6, s), 3);
/// ```
#[inline]
pub fn get_char_index_from_byte_offset(byte_offset: usize, s: &str) -> usize {
    let mut char_index = 0;
    for (pos, _) in String::from(s).char_indices() {
        if byte_offset <= pos {
            return char_index;
        }
        char_index += 1;
    }
    return char_index;
}

/// Returns a vector of all valid bay positions around a station
pub fn get_neighbors(map: &Vec<Vec<char>>, station: &Station) -> Vec<(usize, usize, Direction)> {
    let mut neighbors: Vec<(usize, usize, Direction)> = Vec::new();

    let mut northern_neighbors: Vec<(usize, usize, Direction)> = Vec::new();
    if station.loc.line > 0 {
        for i in 0..station.loc.len {
            if station.loc.col + i < map[station.loc.line - 1].len() {
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
            if station.loc.col + i < map[station.loc.line + 1].len() {
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
