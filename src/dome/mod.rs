use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// Dome shutter state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ShutterState {
    Open = 0,
    Closed = 1,
    Opening = 2,
    Closing = 3,
    Error = 4,
}

/// ASCOM Dome device trait (IDomeV3).
///
/// Controls observatory dome rotation and shutter. Capabilities are individually
/// flagged (`can_find_home`, `can_park`, `can_set_altitude`, `can_set_azimuth`,
/// `can_set_shutter`, `can_slave`, `can_sync_azimuth`).
///
/// Domes can be slaved to a telescope mount so the slit follows the optical path.
pub trait Dome: Device {
    fn altitude(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("altitude".into()))
    }

    fn azimuth(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("azimuth".into()))
    }

    fn at_home(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("at_home".into()))
    }

    fn at_park(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("at_park".into()))
    }

    fn shutter_status(&self) -> AlpacaResult<ShutterState> {
        Err(AlpacaError::NotImplemented("shutter_status".into()))
    }

    fn slaved(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("slaved".into()))
    }

    fn set_slaved(&self, _slaved: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_slaved".into()))
    }

    fn slewing(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("slewing".into()))
    }

    fn can_find_home(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_find_home".into()))
    }

    fn can_park(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_park".into()))
    }

    fn can_set_altitude(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_set_altitude".into()))
    }

    fn can_set_azimuth(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_set_azimuth".into()))
    }

    fn can_set_park(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_set_park".into()))
    }

    fn can_set_shutter(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_set_shutter".into()))
    }

    fn can_slave(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_slave".into()))
    }

    fn can_sync_azimuth(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_sync_azimuth".into()))
    }

    fn slew_to_azimuth(&self, _azimuth: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("slew_to_azimuth".into()))
    }

    fn slew_to_altitude(&self, _altitude: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("slew_to_altitude".into()))
    }

    fn open_shutter(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("open_shutter".into()))
    }

    fn close_shutter(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("close_shutter".into()))
    }

    fn park(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("park".into()))
    }

    fn set_park(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_park".into()))
    }

    fn find_home(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("find_home".into()))
    }

    fn abort_slew(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("abort_slew".into()))
    }

    fn sync_to_azimuth(&self, _azimuth: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("sync_to_azimuth".into()))
    }
}
