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
