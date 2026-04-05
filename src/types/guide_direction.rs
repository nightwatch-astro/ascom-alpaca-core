use serde_repr::{Deserialize_repr, Serialize_repr};

/// Pulse guide direction, shared by Camera and Telescope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GuideDirection {
    /// Guide north (declination positive).
    North = 0,
    /// Guide south (declination negative).
    South = 1,
    /// Guide east (RA positive).
    East = 2,
    /// Guide west (RA negative).
    West = 3,
}
