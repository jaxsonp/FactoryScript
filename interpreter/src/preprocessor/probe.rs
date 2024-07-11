use crate::*;

#[allow(dead_code)]
const BELTS: &str = "─│┌┐└┘═║╔╗╚╝";
const SINGLE_BELTS: &str = "─│┌┐└┘";
const DOUBLE_BELTS: &str = "═║╔╗╚╝";
const NORTH_BELTS: &str = "│└┘║╚╝";
const EAST_BELTS: &str = "─┌└═╔╚";
const SOUTH_BELTS: &str = "│┌┐║╔╗";
const WEST_BELTS: &str = "─┐┘═╗╝";

/// Given a starting position around a station, check if it is an input bay and
/// if it is find the origin of the conveyor belt
///
/// Returns an optional tuple of the origin position if it is an input bay
pub fn evaluate_bay(
    map: &Vec<Vec<char>>,
    start: (usize, usize, Direction),
) -> Option<(usize, usize)> {
    let c = map[start.0][start.1];

    // checking if not a single belt character
    if !SINGLE_BELTS.contains(c) {
        return None;
    }

    // checking if pointing into station
    match start.2 {
        Direction::NORTH => {
            if !SOUTH_BELTS.contains(c) {
                return None;
            }
        }
        Direction::EAST => {
            if !WEST_BELTS.contains(c) {
                return None;
            }
        }
        Direction::SOUTH => {
            if !NORTH_BELTS.contains(c) {
                return None;
            }
        }
        Direction::WEST => {
            if !EAST_BELTS.contains(c) {
                return None;
            }
        }
    }

    let dest = probe(map, start);
    return dest;
}

/// follows a path of block drawing characters until it it hits the end, returning
/// the position of the end if it is a valid conveyor belt
pub fn probe(map: &Vec<Vec<char>>, start: (usize, usize, Direction)) -> Option<(usize, usize)> {
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
            return None;
        }
        debug!(
            4,
            "       - moved to {}:{}, now facing {}", pos.0, pos.1, facing
        );

        // moving to the next char
        match facing {
            Direction::NORTH => {
                if pos.0 == 0 {
                    return None;
                }
                pos.0 -= 1;
            }
            Direction::EAST => {
                pos.1 += 1;
                if pos.1 >= map[pos.0].len() {
                    return None;
                }
            }
            Direction::SOUTH => {
                pos.0 += 1;
                if pos.0 >= map.len() {
                    return None;
                }
            }
            Direction::WEST => {
                if pos.1 == 0 {
                    return None;
                }
                pos.1 -= 1;
            }
        }
        // if the last character was a double belt, we reached the origin
        if DOUBLE_BELTS.contains(c) {
            debug!(4, "       - path ended at {}:{}", pos.0, pos.1);
            return Some(pos);
        }
        // moving
        c = map[pos.0][pos.1];
    }
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
        assert_eq!(probe(&map, (0, 0, Direction::EAST)).unwrap(), (1, 0));
        assert_eq!(probe(&map, (0, 3, Direction::EAST)).unwrap(), (1, 0));
    }

    #[test]
    fn test_probe_dangling() {
        let map = vec![vec!['─', '┐'], vec![' ', '─']];
        assert!(probe(&map, (0, 0, Direction::EAST)).is_none());
        assert!(probe(&map, (1, 1, Direction::WEST)).is_none());
    }

    #[test]
    fn test_probe_out_of_bounds() {
        let map = vec![vec!['─', '┐']];
        assert!(probe(&map, (0, 0, Direction::EAST)).is_none());
        assert!(probe(&map, (0, 1, Direction::NORTH)).is_none());
    }

    #[test]
    fn test_evaluate_bay() {
        let map = vec![
            vec![' ', '│', '┌', '═'],
            vec!['║', '[', ']', ' '],
            vec!['└', '┘', '└', '╝'],
        ];
        assert_eq!(
            evaluate_bay(&map, (2, 1, Direction::SOUTH)).unwrap(),
            (0, 0)
        );
        assert_eq!(
            evaluate_bay(&map, (2, 2, Direction::SOUTH)).unwrap(),
            (1, 3)
        );
        assert!(evaluate_bay(&map, (1, 0, Direction::WEST)).is_none());
        assert!(evaluate_bay(&map, (1, 3, Direction::EAST)).is_none());
        assert!(evaluate_bay(&map, (0, 1, Direction::NORTH)).is_none());
        assert!(evaluate_bay(&map, (0, 2, Direction::NORTH)).is_none());
    }
}
