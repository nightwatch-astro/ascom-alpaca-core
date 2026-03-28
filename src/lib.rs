//! Framework-agnostic ASCOM Alpaca protocol types and traits for Rust.
//!
//! This crate provides the complete ASCOM Alpaca protocol abstraction: typed response
//! envelopes, error codes, device traits for all 10 ASCOM device types (~220 methods),
//! domain enums, device registry, and conformance validation.
//!
//! No HTTP framework, no async runtime — works on ESP32 and desktop.

pub mod device;
pub mod discovery;
pub mod management;
pub mod registry;
pub mod types;

#[cfg(feature = "conformu")]
pub mod conformu;

#[cfg(feature = "camera")]
pub mod camera;
#[cfg(feature = "cover_calibrator")]
pub mod cover_calibrator;
#[cfg(feature = "dome")]
pub mod dome;
#[cfg(feature = "filter_wheel")]
pub mod filter_wheel;
#[cfg(feature = "focuser")]
pub mod focuser;
#[cfg(feature = "observing_conditions")]
pub mod observing_conditions;
#[cfg(feature = "rotator")]
pub mod rotator;
#[cfg(feature = "safety_monitor")]
pub mod safety_monitor;
#[cfg(feature = "switch")]
pub mod switch;
#[cfg(feature = "telescope")]
pub mod telescope;

/// Re-exports of the most commonly used types.
pub mod prelude {
    pub use crate::device::{Device, RegisteredDevice};
    pub use crate::discovery::{
        DiscoveryResponse, DEFAULT_DISCOVERY_PORT, DISCOVERY_PROBE, IPV6_MULTICAST,
    };
    pub use crate::management::{ConfiguredDevice, ServerDescription, ServerInfo};
    pub use crate::registry::{ClientTracker, DeviceRegistry, TransactionCounter};
    pub use crate::types::params::{normalize_params, CommonParams};
    pub use crate::types::{
        AlpacaError, AlpacaResponse, AlpacaResult, DeviceType, MethodResponse, RegistryError,
    };

    #[cfg(feature = "camera")]
    pub use crate::camera::Camera;
    #[cfg(feature = "cover_calibrator")]
    pub use crate::cover_calibrator::CoverCalibrator;
    #[cfg(feature = "dome")]
    pub use crate::dome::Dome;
    #[cfg(feature = "filter_wheel")]
    pub use crate::filter_wheel::FilterWheel;
    #[cfg(feature = "focuser")]
    pub use crate::focuser::Focuser;
    #[cfg(feature = "observing_conditions")]
    pub use crate::observing_conditions::ObservingConditions;
    #[cfg(feature = "rotator")]
    pub use crate::rotator::Rotator;
    #[cfg(feature = "safety_monitor")]
    pub use crate::safety_monitor::SafetyMonitor;
    #[cfg(feature = "switch")]
    pub use crate::switch::Switch;
    #[cfg(feature = "telescope")]
    pub use crate::telescope::Telescope;
}
