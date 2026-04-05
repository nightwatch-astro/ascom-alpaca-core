use serde_repr::{Deserialize_repr, Serialize_repr};

/// Camera operational state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CameraState {
    /// Camera is idle and ready.
    Idle = 0,
    /// Waiting for an exposure to start.
    Waiting = 1,
    /// Sensor is currently exposing.
    Exposing = 2,
    /// Reading data from the sensor.
    Reading = 3,
    /// Downloading image data to the client.
    Download = 4,
    /// An error has occurred.
    Error = 5,
}

/// Camera sensor type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SensorType {
    /// Monochrome (grayscale) sensor.
    Monochrome = 0,
    /// Bayer-pattern color sensor (generic).
    Color = 1,
    /// RGGB Bayer pattern.
    RGGB = 2,
    /// CMYG filter pattern.
    CMYG = 3,
    /// CMYG2 filter pattern.
    CMYG2 = 4,
    /// LRGB filter pattern.
    LRGB = 5,
}

// GuideDirection is in crate::types since it's shared with Telescope.
pub use crate::types::GuideDirection;
