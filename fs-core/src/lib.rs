#[cfg(test)]
mod tests;

/// All valid belt characters
pub const BELTS: &str = "─│┌┐└┘═║╔╗╚╝";

/// All single belt characters
pub const SINGLE_BELTS: &str = "─│┌┐└┘";

/// All double belt characters
pub const DOUBLE_BELTS: &str = "═║╔╗╚╝";

/// All north-connecting belt characters
pub const NORTH_BELTS: &str = "│└┘║╚╝";

/// All east-connecting belt characters
pub const EAST_BELTS: &str = "─┌└═╔╚";

/// All south-connecting belt characters
pub const SOUTH_BELTS: &str = "│┌┐║╔╗";

/// All west-connecting belt characters
pub const WEST_BELTS: &str = "─┐┘═╗╝";

/// Trait to define a station and all the required functionality and information
pub trait StationType {}
