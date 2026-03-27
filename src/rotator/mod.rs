use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// ASCOM Rotator device trait.
pub trait Rotator: Device {
    /// Returns whether the rotator supports the Reverse method.
    fn can_reverse(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_reverse".into()))
    }

    /// Returns whether the rotator is currently moving.
    fn is_moving(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("is_moving".into()))
    }

    /// Returns the raw mechanical position angle (degrees).
    fn mechanical_position(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("mechanical_position".into()))
    }

    /// Returns the current position angle (degrees, with sync offset applied).
    fn position(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("position".into()))
    }

    /// Returns whether rotation direction is reversed.
    fn reverse(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("reverse".into()))
    }

    /// Sets whether rotation direction is reversed.
    fn set_reverse(&self, _reverse: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_reverse".into()))
    }

    /// Returns the minimum step size (degrees).
    fn step_size(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("step_size".into()))
    }

    /// Returns the target position angle (degrees).
    fn target_position(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("target_position".into()))
    }

    /// Immediately halts rotator motion.
    fn halt(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("halt".into()))
    }

    /// Moves the rotator by a relative amount (degrees) from the current position.
    fn r#move(&self, _position: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("move".into()))
    }

    /// Moves to the specified position angle (degrees), relative to sync position.
    fn move_absolute(&self, _position: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("move_absolute".into()))
    }

    /// Moves to the specified raw mechanical position (degrees).
    fn move_mechanical(&self, _position: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("move_mechanical".into()))
    }

    /// Syncs the rotator to the specified position angle (degrees).
    fn sync(&self, _position: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("sync".into()))
    }
}
