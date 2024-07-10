use crate::*;

const BELTS: &str = "─│┌┐└┘═║╔╗╚╝";
const SINGLE_BELTS: &str = "─│┌┐└┘";
const DOUBLE_BELTS: &str = "═║╔╗╚╝";
const NORTH_BELTS: &str = "│└┘║╚╝";
const EAST_BELTS: &str = "─┌└═╔╚";
const SOUTH_BELTS: &str = "│┌┐║╔╗";
const WEST_BELTS: &str = "─┐┘═╗╝";

/// Given a starting position around a station, check if it is an input or output
/// bay, and if it is find its destination
///
/// If successful returns an optional `Bay` struct which contains info
pub fn evaluate_bay(
    map: &Vec<Vec<char>>,
    start: (usize, usize, Direction),
) -> Result<Option<Bay>, Error> {
    let c = map[start.0][start.1];

    // checking if belt character
    if !BELTS.contains(c) {
        return Ok(None); // not a belt
    }

    // checking if pointing into station
    match start.2 {
        Direction::NORTH => {
            if !SOUTH_BELTS.contains(c) {
                return Ok(None);
            }
        }
        Direction::EAST => {
            if !WEST_BELTS.contains(c) {
                return Ok(None);
            }
        }
        Direction::SOUTH => {
            if !NORTH_BELTS.contains(c) {
                return Ok(None);
            }
        }
        Direction::WEST => {
            if !EAST_BELTS.contains(c) {
                return Ok(None);
            }
        }
    }

    if DOUBLE_BELTS.contains(c) {
        // output belt (═)
        let dest = probe(map, start)?;
        return Ok(Some(Bay::Out(dest.0, dest.1)));
    } else {
        // input belt (─)
        return Ok(Some(Bay::In));
    }
}

/// follows a path of block drawing characters until its end
fn probe(map: &Vec<Vec<char>>, start: (usize, usize, Direction)) -> Result<(usize, usize), Error> {
    let mut pos = (start.0, start.1);
    let mut facing = start.2;
    debug!(4, "      probing starting at {}:{}", pos.0, pos.1);

    // error to return if unexpected double belt is found
    let unexpected_double_err = Error {
        t: ErrorType::SyntaxError(SourceLocation {
            line: pos.0,
            col: pos.1,
            len: 1,
        }),
        msg: String::from("Unexpected double conveyor belt"),
    };

    let mut c = map[pos.0][pos.1];
    match facing {
        Direction::NORTH => match c {
            '║' => {}
            '╔' => facing = Direction::EAST,
            '╗' => facing = Direction::WEST,
            _ => panic!(),
        },
        Direction::EAST => match c {
            '═' => {}
            '╝' => facing = Direction::NORTH,
            '╗' => facing = Direction::SOUTH,
            _ => panic!(),
        },
        Direction::SOUTH => match c {
            '║' => {}
            '╚' => facing = Direction::EAST,
            '╝' => facing = Direction::WEST,
            _ => panic!(),
        },
        Direction::WEST => match c {
            '═' => {}
            '╚' => facing = Direction::NORTH,
            '╔' => facing = Direction::SOUTH,
            _ => panic!(),
        },
    }
    loop {
        // checking out next char
        match facing {
            Direction::NORTH => {
                pos.0 -= 1;
            }
            Direction::EAST => {
                pos.1 += 1;
            }
            Direction::SOUTH => {
                pos.0 += 1;
            }
            Direction::WEST => {
                pos.1 -= 1;
            }
        }
        c = map[pos.0][pos.1];

        // checking if it connects to previous char and turning if necessary
        if facing == Direction::NORTH && SOUTH_BELTS.contains(c) {
            match c {
                '│' => {}
                '┌' => facing = Direction::EAST,
                '┐' => facing = Direction::WEST,
                _ => return Err(unexpected_double_err),
            }
        } else if facing == Direction::EAST && WEST_BELTS.contains(c) {
            match c {
                '─' => {}
                '┘' => facing = Direction::NORTH,
                '┐' => facing = Direction::SOUTH,
                _ => return Err(unexpected_double_err),
            }
        } else if facing == Direction::SOUTH && NORTH_BELTS.contains(c) {
            match c {
                '│' => {}
                '└' => facing = Direction::EAST,
                '┘' => facing = Direction::WEST,
                _ => return Err(unexpected_double_err),
            }
        } else if facing == Direction::WEST && EAST_BELTS.contains(c) {
            match c {
                '─' => {}
                '└' => facing = Direction::NORTH,
                '┌' => facing = Direction::SOUTH,
                _ => return Err(unexpected_double_err),
            }
        } else {
            // end of this path
            debug!(
                4,
                "       - path ended at {}:{} with char \'{}\'", pos.0, pos.1, c
            );
            break;
        }
        debug!(
            4,
            "       - moved to {}:{}, facing {}", pos.0, pos.1, facing
        );
    }
    return Ok(pos);
}

/// helper struct to conveniently handle information about a stations bays,
/// whether its an input or output, and its destination pos if its an ouput
pub enum Bay {
    In,
    Out(usize, usize),
}
