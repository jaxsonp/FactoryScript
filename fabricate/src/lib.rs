pub static mut COLOR_OUTPUT: bool = false;
pub static mut DEBUG_LEVEL: u8 = 0;

use std::cmp::min;

pub mod macros;
pub mod preprocessor;
pub mod runtime;
pub mod station;
pub mod stdlib;

use core::StationType;
use station::*;

pub type Namespace = Vec<&'static StationType<'static>>;

pub fn run<'a>(src: &String) -> Result<(), Error> {
    debug!(2, "Initializing namespace...");
    let mut namespace: Namespace = Vec::new();
    for name in (*stdlib::MANIFEST).iter() {
        namespace.push(name);
    }

    debug!(2, "Preprocessing...");
    let lines: Vec<&str> = src.split('\n').collect();
    let (mut stations, start_i, assign_table) = preprocessor::process(&lines, &namespace)?;

    debug!(2, "Starting");
    runtime::execute(&mut stations, start_i, &assign_table)?;
    Ok(())
}

/// Custom error handling struct
#[derive(Debug)]
pub struct Error {
    /// Error type
    ///
    /// named `t` cus `"type"` is a reserved keyword :_(
    pub t: ErrorType,
    /// Location the error originated from
    pub loc: SourceLocation,
    /// Message
    pub msg: String,
}
impl Error {
    /// function for generating a pretty error message
    pub fn pretty_msg(&self, src: &String) -> String {
        // fancy error formatting
        if self.loc == SourceLocation::none() {
            // location in code is N/A
            return format!("{}: {}", self.t, self.msg);
        }
        // generating 2d vector layout of source code
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in src.split('\n').collect::<Vec<&str>>() {
            map.push(line.chars().collect());
        }
        let mut output = format!("{} @ {}\n        ", self.t, self.loc);
        let left_bound = self.loc.col.saturating_sub(24);
        let right_bound = min(80, self.loc.col + self.loc.len + 24);

        for _ in left_bound..self.loc.col {
            output += " ";
        }
        for _ in 0..self.loc.len {
            output += "v";
        }

        // closure to try and get a line of source code to print given an offset
        let try_get_ln = |offset: i32| -> String {
            let line = (self.loc.line as i32) + offset;
            if line < 0 || line as usize >= map.len() {
                return String::new();
            }
            let line = line as usize;
            let left_bound = min(left_bound, map[line].len().saturating_sub(1));
            let right_bound = min(right_bound, map[line].len());
            let mut output = format!("\n \x1b[22m{:>4} | \x1b[2m", line + 1);
            for c in map[line][left_bound..right_bound].iter() {
                output.push(*c);
            }
            return output;
        };

        // printing lines above
        output += try_get_ln(-2).as_str();
        output += try_get_ln(-1).as_str();
        // printing line of error
        {
            let left_bound = min(left_bound, map[self.loc.line].len().saturating_sub(1));
            let right_bound = min(right_bound, map[self.loc.line].len());
            output += format!("\n\x1b[22m-{:->4}-| \x1b[2m", self.loc.line + 1).as_str();
            for c in map[self.loc.line][left_bound..self.loc.col].iter() {
                output.push(*c);
            }
            // bold and underline
            output += "\x1b[22m\x1b[1m\x1b[4m";
            for c in map[self.loc.line][self.loc.col..(self.loc.col + self.loc.len)].iter() {
                output.push(*c);
            }
            output += "\x1b[24m\x1b[2m";
            for c in map[self.loc.line][(self.loc.col + self.loc.len)..right_bound].iter() {
                output.push(*c);
            }
        }
        // printing line below
        output += try_get_ln(1).as_str();
        output += try_get_ln(2).as_str();

        output += "\x1b[22m\n";
        output += self.msg.as_str();
        return output;
    }
}

/// Types of handled errors
#[derive(Debug)]
pub enum ErrorType {
    SyntaxError,
    IdentifierError,
    RuntimeError,
}
impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::SyntaxError => String::from("Syntax Error"),
            Self::IdentifierError => String::from("Identifier Error"),
            Self::RuntimeError => String::from("Runtime Error"),
        };
        write!(f, "{s}")
    }
}

/// Defines the position of a span of characters in the source code, used for
/// syntax parsing and error reporting
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SourceLocation {
    /// line number
    pub line: usize,
    /// column number
    pub col: usize,
    /// length of span
    pub len: usize,
}
impl SourceLocation {
    /// Value to represent if the source location is not applicable
    pub fn none() -> Self {
        Self {
            line: 0,
            col: 0,
            len: 0,
        }
    }
}
impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.line + 1, self.col, self.col + self.len)
    }
}

/// Helper for the cardinal directions
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}
impl std::ops::Not for Direction {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Direction::NORTH => Direction::SOUTH,
            Direction::EAST => Direction::WEST,
            Direction::SOUTH => Direction::NORTH,
            Direction::WEST => Direction::EAST,
        }
    }
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::NORTH => "north",
                Direction::EAST => "east",
                Direction::SOUTH => "south",
                Direction::WEST => "west",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_negation() {
        assert_eq!(!Direction::NORTH, Direction::SOUTH);
        assert_eq!(!Direction::EAST, Direction::WEST);
        assert_eq!(!Direction::SOUTH, Direction::NORTH);
        assert_eq!(!Direction::WEST, Direction::EAST);
    }
}
