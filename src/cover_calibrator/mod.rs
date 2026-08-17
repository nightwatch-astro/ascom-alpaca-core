use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// Calibrator device state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CalibratorState {
    /// No calibrator is present.
    NotPresent = 0,
    /// Calibrator is off.
    Off = 1,
    /// Calibrator is not yet ready (warming up).
    NotReady = 2,
    /// Calibrator is ready and at target brightness.
    Ready = 3,
    /// Calibrator state is unknown.
    Unknown = 4,
    /// Calibrator is in an error state.
    Error = 5,
}

/// Cover device state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CoverState {
    /// No cover is present.
    NotPresent = 0,
    /// Cover is closed.
    Closed = 1,
    /// Cover is moving (opening or closing).
    Moving = 2,
    /// Cover is open.
    Open = 3,
    /// Cover state is unknown.
    Unknown = 4,
    /// Cover is in an error state.
    Error = 5,
}

/// ASCOM `CoverCalibrator` device trait (`ICoverCalibratorV2`).
///
/// Two independent subsystems: a calibrator (flat panel light source for flat frames)
/// and a cover (dust cap). Either or both may be present (`CoverState::NotPresent`,
/// `CalibratorState::NotPresent`). Brightness is 0 to `max_brightness`.
#[allow(missing_docs)] // Trait methods map 1:1 to ASCOM ICoverCalibratorV2
pub trait CoverCalibrator: Device {
    fn brightness(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("brightness".into()))
    }

    fn max_brightness(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("max_brightness".into()))
    }

    fn calibrator_state(&self) -> AlpacaResult<CalibratorState> {
        Err(AlpacaError::NotImplemented("calibrator_state".into()))
    }

    fn cover_state(&self) -> AlpacaResult<CoverState> {
        Err(AlpacaError::NotImplemented("cover_state".into()))
    }

    fn calibrator_on(&self, _brightness: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("calibrator_on".into()))
    }

    fn calibrator_off(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("calibrator_off".into()))
    }

    fn open_cover(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("open_cover".into()))
    }

    fn close_cover(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("close_cover".into()))
    }

    fn halt_cover(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("halt_cover".into()))
    }

    fn calibrator_changing(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("calibrator_changing".into()))
    }

    fn cover_moving(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("cover_moving".into()))
    }
}
