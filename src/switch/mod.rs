use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// ASCOM Switch device trait.
///
/// Multi-channel switch device. All methods take a switch ID parameter
/// to address individual channels.
pub trait Switch: Device {
    /// Returns the number of switch channels (0 to N-1).
    fn max_switch(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("max_switch".into()))
    }

    /// Returns whether the specified switch can be written to.
    fn can_write(&self, _id: u32) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_write".into()))
    }

    /// Returns the state of the specified switch as a boolean.
    fn get_switch(&self, _id: u32) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("get_switch".into()))
    }

    /// Sets the state of the specified switch.
    fn set_switch(&self, _id: u32, _state: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_switch".into()))
    }

    /// Returns the value of the specified switch as a double.
    fn get_switch_value(&self, _id: u32) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("get_switch_value".into()))
    }

    /// Sets the value of the specified switch.
    fn set_switch_value(&self, _id: u32, _value: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_switch_value".into()))
    }

    /// Returns the name of the specified switch.
    fn get_switch_name(&self, _id: u32) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("get_switch_name".into()))
    }

    /// Sets the name of the specified switch.
    fn set_switch_name(&self, _id: u32, _name: &str) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_switch_name".into()))
    }

    /// Returns the description of the specified switch.
    fn get_switch_description(&self, _id: u32) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("get_switch_description".into()))
    }

    /// Returns the minimum value of the specified switch.
    fn min_switch_value(&self, _id: u32) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("min_switch_value".into()))
    }

    /// Returns the maximum value of the specified switch.
    fn max_switch_value(&self, _id: u32) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("max_switch_value".into()))
    }

    /// Returns the step size for the specified switch.
    fn switch_step(&self, _id: u32) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("switch_step".into()))
    }
}
