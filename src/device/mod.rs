pub mod common;

use crate::types::{AlpacaError, AlpacaResult, DeviceType};

/// Base trait for all ASCOM Alpaca devices.
///
/// Every device must provide identity (`static_name`, `unique_id`, `device_type`).
/// All other methods have default implementations that return `NotImplemented`.
///
/// Device traits require `Send + Sync` to enable thread-safe dispatch in server contexts.
pub trait Device: Send + Sync {
    // --- Required methods (no defaults) ---

    /// The device's fixed display name.
    fn static_name(&self) -> &str;

    /// A unique identifier for this device instance.
    fn unique_id(&self) -> &str;

    /// The ASCOM device type.
    fn device_type(&self) -> DeviceType;

    // --- Methods with defaults (return NotImplemented) ---

    fn connected(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("connected".into()))
    }

    fn set_connected(&self, _connected: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_connected".into()))
    }

    fn connecting(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("connecting".into()))
    }

    fn connect(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("connect".into()))
    }

    fn disconnect(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("disconnect".into()))
    }

    fn description(&self) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("description".into()))
    }

    fn driver_info(&self) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("driver_info".into()))
    }

    fn driver_version(&self) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("driver_version".into()))
    }

    fn interface_version(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("interface_version".into()))
    }

    fn name(&self) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("name".into()))
    }

    fn supported_actions(&self) -> AlpacaResult<Vec<String>> {
        Err(AlpacaError::NotImplemented("supported_actions".into()))
    }

    fn action(&self, _action_name: &str, _action_parameters: &str) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("action".into()))
    }

    fn command_blind(&self, _command: &str, _raw: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("command_blind".into()))
    }

    fn command_bool(&self, _command: &str, _raw: bool) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("command_bool".into()))
    }

    fn command_string(&self, _command: &str, _raw: bool) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("command_string".into()))
    }

    fn device_state(&self) -> AlpacaResult<Vec<common::DeviceStateItem>> {
        Err(AlpacaError::NotImplemented("device_state".into()))
    }
}

/// Enum for heterogeneous device storage in the registry.
///
/// Each variant is feature-gated to match its device type module.
pub enum RegisteredDevice {
    #[cfg(feature = "safety_monitor")]
    SafetyMonitor(Box<dyn crate::safety_monitor::SafetyMonitor>),
    #[cfg(feature = "switch")]
    Switch(Box<dyn crate::switch::Switch>),
    #[cfg(feature = "camera")]
    Camera(Box<dyn crate::camera::Camera>),
    #[cfg(feature = "cover_calibrator")]
    CoverCalibrator(Box<dyn crate::cover_calibrator::CoverCalibrator>),
    #[cfg(feature = "dome")]
    Dome(Box<dyn crate::dome::Dome>),
    #[cfg(feature = "filter_wheel")]
    FilterWheel(Box<dyn crate::filter_wheel::FilterWheel>),
    #[cfg(feature = "focuser")]
    Focuser(Box<dyn crate::focuser::Focuser>),
    #[cfg(feature = "observing_conditions")]
    ObservingConditions(Box<dyn crate::observing_conditions::ObservingConditions>),
    #[cfg(feature = "rotator")]
    Rotator(Box<dyn crate::rotator::Rotator>),
    #[cfg(feature = "telescope")]
    Telescope(Box<dyn crate::telescope::Telescope>),
    /// Hidden variant to ensure the enum is never empty when no device features are enabled.
    #[doc(hidden)]
    _None(std::convert::Infallible),
}

impl RegisteredDevice {
    /// Returns a reference to the underlying `Device` trait object.
    pub fn as_device(&self) -> &dyn Device {
        match self {
            #[cfg(feature = "safety_monitor")]
            Self::SafetyMonitor(d) => d.as_ref() as &dyn Device,
            #[cfg(feature = "switch")]
            Self::Switch(d) => d.as_ref() as &dyn Device,
            #[cfg(feature = "camera")]
            Self::Camera(d) => d.as_ref() as &dyn Device,
            #[cfg(feature = "cover_calibrator")]
            Self::CoverCalibrator(d) => d.as_ref() as &dyn Device,
            #[cfg(feature = "dome")]
            Self::Dome(d) => d.as_ref() as &dyn Device,
            #[cfg(feature = "filter_wheel")]
            Self::FilterWheel(d) => d.as_ref() as &dyn Device,
            #[cfg(feature = "focuser")]
            Self::Focuser(d) => d.as_ref() as &dyn Device,
            #[cfg(feature = "observing_conditions")]
            Self::ObservingConditions(d) => d.as_ref() as &dyn Device,
            #[cfg(feature = "rotator")]
            Self::Rotator(d) => d.as_ref() as &dyn Device,
            #[cfg(feature = "telescope")]
            Self::Telescope(d) => d.as_ref() as &dyn Device,
            Self::_None(infallible) => match *infallible {},
        }
    }

    /// Returns the device type for this registered device.
    pub fn device_type(&self) -> DeviceType {
        self.as_device().device_type()
    }
}

// --- From implementations for ergonomic registration ---

#[cfg(feature = "safety_monitor")]
impl From<Box<dyn crate::safety_monitor::SafetyMonitor>> for RegisteredDevice {
    fn from(d: Box<dyn crate::safety_monitor::SafetyMonitor>) -> Self {
        Self::SafetyMonitor(d)
    }
}

#[cfg(feature = "switch")]
impl From<Box<dyn crate::switch::Switch>> for RegisteredDevice {
    fn from(d: Box<dyn crate::switch::Switch>) -> Self {
        Self::Switch(d)
    }
}

#[cfg(feature = "camera")]
impl From<Box<dyn crate::camera::Camera>> for RegisteredDevice {
    fn from(d: Box<dyn crate::camera::Camera>) -> Self {
        Self::Camera(d)
    }
}

#[cfg(feature = "cover_calibrator")]
impl From<Box<dyn crate::cover_calibrator::CoverCalibrator>> for RegisteredDevice {
    fn from(d: Box<dyn crate::cover_calibrator::CoverCalibrator>) -> Self {
        Self::CoverCalibrator(d)
    }
}

#[cfg(feature = "dome")]
impl From<Box<dyn crate::dome::Dome>> for RegisteredDevice {
    fn from(d: Box<dyn crate::dome::Dome>) -> Self {
        Self::Dome(d)
    }
}

#[cfg(feature = "filter_wheel")]
impl From<Box<dyn crate::filter_wheel::FilterWheel>> for RegisteredDevice {
    fn from(d: Box<dyn crate::filter_wheel::FilterWheel>) -> Self {
        Self::FilterWheel(d)
    }
}

#[cfg(feature = "focuser")]
impl From<Box<dyn crate::focuser::Focuser>> for RegisteredDevice {
    fn from(d: Box<dyn crate::focuser::Focuser>) -> Self {
        Self::Focuser(d)
    }
}

#[cfg(feature = "observing_conditions")]
impl From<Box<dyn crate::observing_conditions::ObservingConditions>> for RegisteredDevice {
    fn from(d: Box<dyn crate::observing_conditions::ObservingConditions>) -> Self {
        Self::ObservingConditions(d)
    }
}

#[cfg(feature = "rotator")]
impl From<Box<dyn crate::rotator::Rotator>> for RegisteredDevice {
    fn from(d: Box<dyn crate::rotator::Rotator>) -> Self {
        Self::Rotator(d)
    }
}

#[cfg(feature = "telescope")]
impl From<Box<dyn crate::telescope::Telescope>> for RegisteredDevice {
    fn from(d: Box<dyn crate::telescope::Telescope>) -> Self {
        Self::Telescope(d)
    }
}
