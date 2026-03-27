use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// ASCOM FilterWheel device trait.
pub trait FilterWheel: Device {
    /// Returns the current filter wheel position (0-based).
    /// Returns -1 if the wheel is moving.
    fn position(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("position".into()))
    }

    /// Sets the filter wheel position (0-based).
    fn set_position(&self, _position: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_position".into()))
    }

    /// Returns the filter names.
    fn names(&self) -> AlpacaResult<Vec<String>> {
        Err(AlpacaError::NotImplemented("names".into()))
    }

    /// Returns the focus offsets for each filter.
    fn focus_offsets(&self) -> AlpacaResult<Vec<i32>> {
        Err(AlpacaError::NotImplemented("focus_offsets".into()))
    }
}
