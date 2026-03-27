use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// ASCOM Focuser device trait.
pub trait Focuser: Device {
    /// Returns whether the focuser supports absolute positioning.
    fn absolute(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("absolute".into()))
    }

    /// Returns whether the focuser is currently moving.
    fn is_moving(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("is_moving".into()))
    }

    /// Returns the maximum increment size (steps).
    fn max_increment(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("max_increment".into()))
    }

    /// Returns the maximum step position.
    fn max_step(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("max_step".into()))
    }

    /// Returns the current position (steps).
    fn position(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("position".into()))
    }

    /// Returns the step size in microns.
    fn step_size(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("step_size".into()))
    }

    /// Returns the focuser temperature (°C).
    fn temperature(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("temperature".into()))
    }

    /// Returns whether temperature compensation is active.
    fn temp_comp(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("temp_comp".into()))
    }

    /// Sets whether temperature compensation is active.
    fn set_temp_comp(&self, _enabled: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_temp_comp".into()))
    }

    /// Returns whether temperature compensation is available.
    fn temp_comp_available(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("temp_comp_available".into()))
    }

    /// Immediately halts focuser motion.
    fn halt(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("halt".into()))
    }

    /// Moves the focuser to the specified position.
    fn r#move(&self, _position: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("move".into()))
    }
}
