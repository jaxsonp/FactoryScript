use core::Pallet;
use std::collections::HashMap;

use crate::*;

#[cfg(test)]
mod tests;

/// Spawns pallets from the start station and starts the execution loop
pub fn execute(
    stations: &Vec<Station>,
    start_i: usize,
    assign_table: &HashMap<usize, Pallet>,
) -> Result<(), Error> {
    Ok(())
}
