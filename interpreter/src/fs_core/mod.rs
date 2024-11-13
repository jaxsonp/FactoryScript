
pub mod constants;
pub mod stations;

pub const BELT_CHARS: &str = "─│┌┐└┘═║╔╗╚╝";
pub const SINGLE_BELT_CHARS: &str = "─│┌┐└┘";
pub const DOUBLE_BELT_CHARS: &str = "═║╔╗╚╝";
pub const NORTH_BELT_CHARS: &str = "│└┘║╚╝";
pub const EAST_BELT_CHARS: &str = "─┌└═╔╚";
pub const SOUTH_BELT_CHARS: &str = "│┌┐║╔╗";
pub const WEST_BELT_CHARS: &str = "─┐┘═╗╝";

/// Defines a station and all the required information and functionality
#[derive(Debug)]
pub struct StationType {
    /// Identifier
    pub id: &'static str,
    /// Alternate identifier
    pub alt_id: Option<&'static str>,
    /// Minimum number of inputs required for this station to trigger its procedure
    pub inputs: usize,
    /// Does this station produce an output pallet
    pub output: bool,
    /// Station's procedure, takes a vector of input pallets and returns an optional
    /// pallet if successful, and an error message in a String if not
    pub procedure: fn(pallets: &Vec<Option<Pallet>>) -> Result<Option<Pallet>, String>,

    pub description: &'static str,
}
impl StationType {
    /// Function to check whether a station has a certain ID
    pub fn has_id(&self, query: &str) -> bool {
        return self.id == query || (self.alt_id.is_some_and(|alt_id| alt_id == query));
    }
}

/// Instance of a pallet
#[derive(Debug, Clone, PartialEq)]
pub enum Pallet {
    Empty,
    Bool(bool),
    Char(char),
    String(String),
    Int(i64),
    Float(f64),
}
impl std::fmt::Display for Pallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pallet::Empty => String::from("Pallet< >"),
                Pallet::Bool(b) => format!("Pallet<b:{}>", if *b { "true" } else { "false" }),
                Pallet::Char(c) => format!("Pallet<c:\'{}\'>", c),
                Pallet::String(s) => format!("Pallet<s:\"{}\">", s),
                Pallet::Int(i) => format!("Pallet<i:{}>", i),
                Pallet::Float(f) => format!("Pallet<f:{}>", f),
            },
        )
    }
}

/* BROKEN
want to fix but I can't figure out the procedure definition macro stuff
because of macro hygeine

/// Macro to define a station in a library.
#[macro_export]
macro_rules! define_station {
    (
        $name:ident {
            id: $id:literal,
            inputs: $inputs:expr,
            output: $output:expr,
            procedure: $procedure:item
        }
    ) => {
        paste! {

            $procedure
            #[allow(non_upper_case_globals)]
            static [<$name>]: StationType = StationType {
                id: $id,
                inputs: $inputs,
                output: $output,
                procedure: [<$name:snake _procedure>],
            };
        }
    };
}
*/
