use serde_repr::{Deserialize_repr, Serialize_repr};

/// Camera operational state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CameraState {
    Idle = 0,
    Waiting = 1,
    Exposing = 2,
    Reading = 3,
    Download = 4,
    Error = 5,
}

/// Camera sensor type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SensorType {
    Monochrome = 0,
    Color = 1,
    RGGB = 2,
    CMYG = 3,
    CMYG2 = 4,
    LRGB = 5,
}

/// Pulse guide direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum GuideDirection {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}
