use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// Calibrator device state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CalibratorState {
    NotPresent = 0,
    Off = 1,
    NotReady = 2,
    Ready = 3,
    Unknown = 4,
    Error = 5,
}

/// Cover device state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CoverState {
    NotPresent = 0,
    Closed = 1,
    Moving = 2,
    Open = 3,
    Unknown = 4,
    Error = 5,
}

/// ASCOM CoverCalibrator device trait.
///
/// Two independent subsystems: a calibrator (flat panel) and a cover (dust cap).
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
