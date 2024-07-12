use crate::*;
use fs_core::*;

/// Given a starting position around a station, check if it is an input bay and
/// if it is find the origin of the conveyor belt
///
/// Returns an optional tuple of the origin position if it is an input bay
pub fn evaluate_bay(
    map: &Vec<Vec<char>>,
    start: (usize, usize, Direction),
) -> Result<Option<(usize, usize)>, Error> {
    let c = map[start.0][start.1];

    // checking if not a single belt character
    if !SINGLE_BELTS.contains(c) {
        return Ok(None);
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

    let dest = probe(map, start)?;
    return Ok(Some(dest));
}

/// follows a path of block drawing characters until it it hits the end, returning
/// the position of the end if it is a valid conveyor belt
pub fn probe(
    map: &Vec<Vec<char>>,
    start: (usize, usize, Direction),
) -> Result<(usize, usize), Error> {
    let mut pos = (start.0, start.1);
    let mut facing = start.2;
    debug!(4, "      probing starting at {}:{}", pos.0, pos.1);

    let mut c = map[pos.0][pos.1];
    loop {
        // checking if current char connects to previous char and turning
        if facing == Direction::NORTH && SOUTH_BELTS.contains(c) {
            match c {
                '│' | '║' => {}
                '┌' | '╔' => facing = Direction::EAST,
                '┐' | '╗' => facing = Direction::WEST,
                _ => panic!(),
            }
        } else if facing == Direction::EAST && WEST_BELTS.contains(c) {
            match c {
                '─' | '═' => {}
                '┘' | '╝' => facing = Direction::NORTH,
                '┐' | '╗' => facing = Direction::SOUTH,
                _ => panic!(),
            }
        } else if facing == Direction::SOUTH && NORTH_BELTS.contains(c) {
            match c {
                '│' | '║' => {}
                '└' | '╚' => facing = Direction::EAST,
                '┘' | '╝' => facing = Direction::WEST,
                _ => panic!(),
            }
        } else if facing == Direction::WEST && EAST_BELTS.contains(c) {
            match c {
                '─' | '═' => {}
                '└' | '╚' => facing = Direction::NORTH,
                '┌' | '╔' => facing = Direction::SOUTH,
                _ => panic!(),
            }
        } else {
            // dangling belt
            return Err(Error {
                t: ErrorType::SyntaxError,
                loc: SourceLocation {
                    line: pos.0,
                    col: pos.1,
                    len: 1,
                },
                msg: String::from("Dangling belt"),
            });
        }
        debug!(
            4,
            "       - moved to {}:{}, now facing {}", pos.0, pos.1, facing
        );

        // moving to the next char
        match facing {
            Direction::NORTH => {
                if pos.0 == 0 {
                    break;
                }
                pos.0 -= 1;
            }
            Direction::EAST => {
                pos.1 += 1;
                if pos.1 >= map[pos.0].len() {
                    break;
                }
            }
            Direction::SOUTH => {
                pos.0 += 1;
                if pos.0 >= map.len() {
                    break;
                }
            }
            Direction::WEST => {
                if pos.1 == 0 {
                    break;
                }
                pos.1 -= 1;
            }
        }
        // if the last character was a double belt, we reached the origin
        if DOUBLE_BELTS.contains(c) {
            debug!(4, "       - path ended at {}:{}", pos.0, pos.1);
            return Ok(pos);
        }
        // moving
        c = map[pos.0][pos.1];
    }

    // dangling belt out of bounds
    return Err(Error {
        t: ErrorType::SyntaxError,
        loc: SourceLocation {
            line: pos.0,
            col: pos.1,
            len: 1,
        },
        msg: String::from("Dangling belt, out of bounds"),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe() {
        let map = vec![
            vec!['─', '─', '─', '┐'],
            vec![' ', '┌', '┐', '│'],
            vec!['╚', '┘', '└', '┘'],
        ];
        assert_eq!(probe(&map, (0, 0, Direction::EAST)).ok().unwrap(), (1, 0));
        assert_eq!(probe(&map, (0, 3, Direction::EAST)).ok().unwrap(), (1, 0));
    }

    #[test]
    fn test_probe_dangling() {
        let map = vec![vec!['─', '┐'], vec![' ', '─']];
        assert!(probe(&map, (0, 0, Direction::EAST)).is_err());
        assert!(probe(&map, (1, 1, Direction::WEST)).is_err());
    }

    #[test]
    fn test_probe_out_of_bounds() {
        let map = vec![vec!['─', '┐']];
        assert!(probe(&map, (0, 0, Direction::EAST)).is_err());
        assert!(probe(&map, (0, 1, Direction::NORTH)).is_err());
    }
}
