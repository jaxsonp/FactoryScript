use crate::*;

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
    pub fn new<S: Into<String>>(t: ErrorType, loc: SourceLocation, msg: S) -> Self {
        Self {
            t,
            loc,
            msg: msg.into(),
        }
    }

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
