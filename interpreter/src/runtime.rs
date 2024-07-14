use core::Pallet;
use std::collections::HashMap;

use crate::*;

/// Spawns pallets from the start station and starts the execution loop
pub fn execute(
    stations: &mut Vec<Station>,
    start_i: usize,
    assign_table: &HashMap<usize, Pallet>,
) -> Result<(), Error> {
    // Vector of all pallets to move in the next step, tuple with the pallet and
    // the destination index and bay number
    let mut moving_pallets: Vec<(Pallet, (usize, usize))> = Vec::new();

    // begin from start station
    let start_station = &stations[start_i];
    debug!(3, "Start pallet spawned at #{start_i}");
    moving_pallets.push((Pallet::Empty, start_station.out_bays[0]));
    let mut t: usize = 0;
    while !moving_pallets.is_empty() {
        // moving the pallets
        for (pallet, dest) in moving_pallets.iter() {
            if stations[dest.0].in_bays[dest.1].is_some() {
                return Err(Error {
                    t: ErrorType::RuntimeError,
                    loc: stations[dest.0].loc,
                    msg: format!("Station input bay {} is already occupied", dest.1),
                });
            }
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
                if station.logic.has_id("assign") {
                    // special case: assign station
                    let new_pallet = if let Some(pallet) = assign_table.get(&i) {
                        pallet
                    } else {
                        return Err(Error {
                            t: ErrorType::RuntimeError,
                            loc: station.loc,
                            msg: format!("Can't find assign table entry for #{i}"),
                        });
                    };
                    debug!(4, "    - Produced: {}", new_pallet);
                    moving_pallets.push((new_pallet.clone(), station.out_bays[0]));
                    station.clear_in_bays();
                    continue;
                } else if station.logic.has_id("joint") {
                    // special case: joint station
                    for in_bay in station.in_bays.iter() {
                        if let Some(pallet) = in_bay {
                            debug!(4, "    - Produced: {}", pallet);
                            for out_bay in station.out_bays.iter() {
                                moving_pallets.push((pallet.clone(), *out_bay));
                            }
                        }
                    }
                    station.clear_in_bays();
                    continue;
                }

                // executing general procedures
                let procedure = station.logic.procedure;
                match procedure(&station.in_bays) {
                    Ok(Some(p)) => {
                        if !station.logic.output {
                            return Err(Error {
                                t: ErrorType::RuntimeError,
                                loc: station.loc,
                                msg: String::from("Station procedure returned pallet unexpectedly"),
                            });
                        }
                        debug!(4, "    - produced: {}", p);
                        moving_pallets.push((p, station.out_bays[0]));
                    }
                    Ok(None) => {
                        if station.logic.output {
                            return Err(Error {
                                t: ErrorType::RuntimeError,
                                loc: station.loc,
                                msg: String::from(
                                    "Station procedured did not return pallet as expected",
                                ),
                            });
                        }
                        debug!(4, "    - produced: None",);
                    }
                    Err(msg) => {
                        return Err(Error {
                            t: ErrorType::RuntimeError,
                            loc: station.loc,
                            msg,
                        });
                    }
                }

                station.clear_in_bays();
            }
        }
        debug!(3, "Step {t} completed ( ms)");
        t += 1;
    }
    debug!(2, "No remaining moving pallets");
    return Ok(());
}
