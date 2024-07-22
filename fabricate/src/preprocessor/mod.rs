use core::Pallet;
use std::collections::HashMap;

pub mod _parse;
pub mod probe;
pub mod station_parser;
#[cfg(test)]
mod tests;

use crate::*;

/// Preprocesses a source string, validating the syntax and grammar
///
/// Returns a tuple containing a vector of stations and the assignment table, which
/// store the index of every assign station and its corresponding assign value
pub fn process<'a>(
    src: &str,
    ns: &Namespace,
) -> Result<(Vec<Station>, usize, HashMap<usize, Pallet>), Error> {
    // generating 2d vector layout of source code
    let mut char_map: Vec<Vec<char>> = Vec::new();
    let mut n_chars = 0;
    for line in src.split('\n') {
        let row = line.chars().collect::<Vec<char>>();
        n_chars += row.len();
        char_map.push(row);
    }
    if n_chars == 0 {
        return Err(Error::new(SyntaxError, SourcePos::zero(), "Empty file"));
    }

    // station discovery
    debug!(3, "Discovering stations");
    let (mut stations, assign_table) = station_parser::parse_stations(&char_map, ns)?;
    debug!(3, "Found {} stations", stations.len());

    // getting start station's index
    let mut start_i: usize = 0;
    let mut found_start = false;
    for i in 0..stations.len() {
        if stations[i].logic.id == "start" {
            if found_start {
                return Err(Error::new(
                    SyntaxError,
                    stations[i].loc,
                    "Found multiple start stations",
                ));
            }
            start_i = i;
            found_start = true;
            break;
        }
    }
    if !found_start {
        return Err(Error::new(
            SyntaxError,
            SourcePos::zero(),
            "Unable to locate start station",
        ));
    }

    debug!(2, "Finished preprocessing");
    Ok((stations, 0, assign_table))
}

/*
pub fn get_neighbors(map: &Vec<Vec<char>>, station: &Station) -> Vec<(usize, usize, Direction)> {
    let mut neighbors: Vec<(usize, usize, Direction)> = Vec::new();

    let mut northern_neighbors: Vec<(usize, usize, Direction)> = Vec::new();
    if station.loc.pos.line > 0 {
        for i in 0..station.loc.len {
            if station.loc.pos.col + i < map[station.loc.pos.line - 1].len() {
                northern_neighbors.push((
                    station.loc.pos.line - 1,
                    station.loc.pos.col + i,
                    Direction::NORTH,
                ));
            }
        }
    }
    let mut eastern_neighbors: Vec<(usize, usize, Direction)> = Vec::new();
    if station.loc.pos.col + station.loc.len < map[station.loc.pos.line].len() {
        eastern_neighbors.push((
            station.loc.pos.line,
            station.loc.pos.col + station.loc.len,
            Direction::EAST,
        ));
    }
    let mut southern_neighbors: Vec<(usize, usize, Direction)> = Vec::new();
    if station.loc.pos.line < (map.len() - 1) {
        for i in (0..station.loc.len).rev() {
            if station.loc.pos.col + i < map[station.loc.pos.line + 1].len() {
                southern_neighbors.push((
                    station.loc.pos.line + 1,
                    station.loc.pos.col + i,
                    Direction::SOUTH,
                ));
            }
        }
    }
    let mut western_neighbors: Vec<(usize, usize, Direction)> = Vec::new();
    if station.loc.pos.col > 0 {
        western_neighbors.push((
            station.loc.pos.line,temporal
            station.loc.pos.col - 1,
            Direction::WEST,
        ));
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
} */
