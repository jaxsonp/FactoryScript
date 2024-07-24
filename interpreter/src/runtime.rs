use core::Pallet;
use std::collections::HashMap;

use crate::*;

/// Spawns pallets from the start station and starts the execution loop, returns
/// the number of steps in the program
pub fn execute(
    stations: &mut Vec<Station>,
    start_i: usize,
    assign_table: &HashMap<usize, Pallet>,
) -> Result<usize, Error> {
    // Vector of all pallets to move in the next step, tuple with the pallet and
    // the destination index and bay number
    let mut moving_pallets: Vec<(Pallet, (usize, usize))> = Vec::new();

    // begin from start station
    let start_station = &stations[start_i];
    for out_bay in start_station.out_bays.iter() {
        moving_pallets.push((Pallet::Empty, *out_bay));
    }
    debug!(3, "Start pallets spawned at #{start_i}");

    let mut step_count: usize = 0;
    'execution_loop: while !moving_pallets.is_empty() {
        // recording start time of iteration
        let step_start_t = Instant::now();

        // moving the pallets
        for (pallet, dest) in moving_pallets.iter() {
            debug!(3, " - pallet moved to #{}:{} ({})", dest.0, dest.1, pallet);
            stations[dest.0].in_bays[dest.1] = Some(pallet.clone());
        }
        moving_pallets.clear();
        // executing station procedures
        for i in 0..stations.len() {
            let station = &mut stations[i];
            // counting occupied bays
            let mut occupied_bays = 0;
            for bay in station.in_bays.iter() {
                if bay.is_some() {
                    occupied_bays += 1;
                }
            }
            if occupied_bays >= station.logic.inputs && station.logic.inputs > 0 {
                // running procedures
                debug!(3, " - Procedure triggered on #{i} ({})", station.logic.id);
                // handling special case stations
                if station.logic.id == "assign" {
                    // special case: assign station
                    if let Some(p) = assign_table.get(&i) {
                        debug!(4, "    - Produced: {}", p);
                        for out_bay in station.out_bays.iter() {
                            moving_pallets.push((p.clone(), *out_bay));
                        }
                    } else {
                        return Err(Error::new(
                            RuntimeError,
                            station.loc,
                            format!("Can't find assign table entry for #{i}"),
                        ));
                    };
                    station.clear_in_bays();
                    continue;
                } else if station.logic.id == "joint" {
                    // special case: joint station
                    for in_bay in station.in_bays.iter() {
                        if let Some(p) = in_bay {
                            debug!(4, "    - Produced: {}", p);
                            for out_bay in station.out_bays.iter() {
                                moving_pallets.push((p.clone(), *out_bay));
                            }
                            break;
                        }
                    }
                    station.clear_in_bays();
                    continue;
                } else if station.logic.id == "exit" {
                    // special case: exit
                    debug!(2, "No remaining moving pallets");
                    break 'execution_loop;
                }

                // executing general procedures
                let procedure = station.logic.procedure;
                match procedure(&station.in_bays) {
                    Ok(Some(p)) => {
                        if !station.logic.output {
                            return Err(Error::new(
                                RuntimeError,
                                station.loc,
                                "Station procedure returned pallet unexpectedly",
                            ));
                        }
                        debug!(4, "    - produced: {}", p);
                        for out_bay in station.out_bays.iter() {
                            moving_pallets.push((p.clone(), *out_bay));
                        }
                    }
                    Ok(None) => {
                        if station.logic.output
                            && station.logic.id != "gate"
                            && station.logic.id != "filter"
                        {
                            return Err(Error::new(
                                RuntimeError,
                                station.loc,
                                "Station procedured did not return pallet as expected",
                            ));
                        }
                        debug!(4, "    - produced: None",);
                    }
                    Err(msg) => {
                        return Err(Error::new(RuntimeError, station.loc, msg));
                    }
                }

                station.clear_in_bays();
            }
        }
        debug!(
            3,
            "Step {step_count} completed ({:.3} ms)",
            step_start_t.elapsed().as_secs_f64() * 1000.0
        );
        step_count += 1;
    }
    debug!(2, "No remaining moving pallets");

    return Ok(step_count);
}
