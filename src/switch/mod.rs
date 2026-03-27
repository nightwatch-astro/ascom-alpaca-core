use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// ASCOM Switch device trait (ISwitchV3).
///
/// Multi-channel device supporting boolean (on/off), multi-state (selector),
/// and analog (continuous range) channels. All methods take a switch `id` parameter
/// (0 to `max_switch - 1`) to address individual channels.
///
/// Each channel has: min/max range, step size, read/write capability, and a name.
/// Boolean channels use min=0, max=1, step=1. Analog channels use finer steps.
///
/// ISwitchV3 adds optional async methods: `can_async`, `set_async`, `set_async_value`,
/// `cancel_async`, `state_change_complete`.
///
/// # Example
///
/// ```rust
/// use ascom_alpaca_core::prelude::*;
/// use ascom_alpaca_core::switch::Switch;
///
/// # struct MySwitch;
/// # impl Device for MySwitch {
/// #     fn static_name(&self) -> &str { "Sw" }
/// #     fn unique_id(&self) -> &str { "sw-001" }
/// #     fn device_type(&self) -> DeviceType { DeviceType::Switch }
/// # }
/// impl Switch for MySwitch {
///     fn max_switch(&self) -> AlpacaResult<i32> { Ok(2) }
///     fn can_write(&self, id: u32) -> AlpacaResult<bool> { Ok(true) }
///     fn get_switch(&self, id: u32) -> AlpacaResult<bool> { Ok(false) }
///     fn set_switch(&self, id: u32, state: bool) -> AlpacaResult<()> { Ok(()) }
///     fn get_switch_value(&self, id: u32) -> AlpacaResult<f64> { Ok(0.0) }
///     fn set_switch_value(&self, id: u32, value: f64) -> AlpacaResult<()> { Ok(()) }
///     fn min_switch_value(&self, id: u32) -> AlpacaResult<f64> { Ok(0.0) }
///     fn max_switch_value(&self, id: u32) -> AlpacaResult<f64> { Ok(1.0) }
///     fn switch_step(&self, id: u32) -> AlpacaResult<f64> { Ok(1.0) }
/// }
/// ```
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

    // --- ISwitchV3 async methods ---

    /// Returns whether the specified switch supports asynchronous state changes.
    fn can_async(&self, _id: u32) -> AlpacaResult<bool> {
        Ok(false)
    }

    /// Asynchronously sets the boolean state of the specified switch.
    fn set_async(&self, _id: u32, _state: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_async".into()))
    }

    /// Asynchronously sets the value of the specified switch.
    fn set_async_value(&self, _id: u32, _value: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_async_value".into()))
    }

    /// Cancels an in-progress asynchronous state change on the specified switch.
    fn cancel_async(&self, _id: u32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("cancel_async".into()))
    }

    /// Returns whether the most recent asynchronous operation on the specified switch has completed.
    fn state_change_complete(&self, _id: u32) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("state_change_complete".into()))
    }
}
