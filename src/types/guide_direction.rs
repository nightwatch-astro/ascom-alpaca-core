use serde_repr::{Deserialize_repr, Serialize_repr};

/// Pulse guide direction, shared by Camera and Telescope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GuideDirection {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}
