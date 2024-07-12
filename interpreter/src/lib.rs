pub static mut COLOR_OUTPUT: bool = false;
pub static mut DEBUG_LEVEL: u8 = 0;

pub mod macros;
pub mod preprocessor;
pub mod runtime;
pub mod station;
pub mod stdlib;

use core::StationType;
use station::*;

pub type Namespace = Vec<&'static StationType<'static>>;

pub fn run<'a>(src: String) -> Result<(), Error> {
    debug!(2, "Initializing stdlib...");
    let mut namespace: Namespace = Vec::new();
    for name in (*stdlib::NAMESPACE).iter() {
        namespace.push(name);
    }

    debug!(2, "Preprocessing...");
    let lines: Vec<&str> = src.split('\n').collect();
    let (mut stations, start_i, assign_table) = preprocessor::process(&lines, &namespace)?;

    debug!(1, "Starting...");
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
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.t, self.msg)
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
impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} +{}", self.line, self.col, self.len)
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
            Direction::SOUTH => Direction::SOUTH,
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
