use core::Pallet;
use std::collections::HashMap;

pub mod parse;
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
    let (mut stations, start_index, assign_table) = parse::discover_stations(lines, ns)?;
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
        for neighbor in parse::get_neighbors(&map, &mut stations[i]) {
            // checking if each neighbor is an input bay and finding the origin pos of the conveyor belt
            if let Some(origin_pos) = probe::evaluate_bay(&map, neighbor)? {
                // finding station at the origin position
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
                        msg: String::from("Conveyor belt origin detached, expected station here"),
                    });
                }
                let origin_station_i = origin_station.unwrap();

                // checking if the origin station has multiple output bays (except joints)
                if stations[origin_station_i].out_bays.len() >= 1
                    && stations[origin_station_i].logic.id != "joint"
                {
                    return Err(Error {
                        t: ErrorType::SyntaxError,
                        loc: stations[origin_station_i].loc,
                        msg: format!("Too many output bays, expected 1"),
                    });
                }

                // checking if there are too many input bays (except joints)
                if stations[i].in_bays.len() >= stations[i].logic.inputs
                    && stations[i].logic.id != "joint"
                {
                    return Err(Error {
                        t: ErrorType::SyntaxError,
                        loc: stations[i].loc,
                        msg: format!("Too many input bays, expected {}", stations[i].logic.inputs),
                    });
                }
                let in_bay_i = stations[i].in_bays.len();
                stations[origin_station_i].out_bays.push((i, in_bay_i));
                stations[i].in_bays.push(None);
                debug!(3, "    - from #{origin_station_i} to bay {in_bay_i}");
            }
        }
        // checking if station has required number of inputs
        if stations[i].in_bays.len() < stations[i].logic.inputs && stations[i].logic.id != "joint" {
            return Err(Error {
                t: ErrorType::SyntaxError,
                loc: stations[i].loc,
                msg: format!(
                    "Missing input bays, expected {}, found {}",
                    stations[i].logic.inputs,
                    stations[i].in_bays.len()
                ),
            });
        // checking if joint station has at least one input
        } else if stations[i].logic.id == "joint" && stations[i].in_bays.len() < 1 {
            return Err(Error {
                t: ErrorType::SyntaxError,
                loc: stations[i].loc,
                msg: format!(
                    "Joint station expects at least 1 input bay, found {}",
                    stations[i].in_bays.len()
                ),
            });
        }
    }

    // making sure the stations have outputs if they need them
    for station in stations.iter() {
        if station.logic.output && station.out_bays.len() < 1 {
            return Err(Error {
                t: ErrorType::SyntaxError,
                loc: station.loc,
                msg: String::from("Missing output bay"),
            });
        }
        if !station.logic.output && station.out_bays.len() > 0 {
            return Err(Error {
                t: ErrorType::SyntaxError,
                loc: station.loc,
                msg: String::from("Unexpected output bay"),
            });
        }
    }

    debug!(2, "Finished preprocessing");
    Ok((stations, start_index, assign_table))
}
