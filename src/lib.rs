#![warn(missing_docs)]
#![warn(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    clippy::todo,
    clippy::unimplemented,
    clippy::dbg_macro,
    clippy::undocumented_unsafe_blocks,
    clippy::missing_assert_message
)]
//! Framework-agnostic ASCOM Alpaca protocol types and traits for Rust.
//!
//!
//! This crate provides the complete ASCOM Alpaca protocol abstraction: typed response
//! envelopes, error codes, device traits for all 10 ASCOM device types (~220 methods),
//! domain enums, device registry, and conformance validation.
//!
//! No HTTP framework, no async runtime — works on ESP32 and desktop.

/// Base device trait and heterogeneous device storage.
pub mod device;
/// Alpaca UDP discovery protocol.
pub mod discovery;
/// Server management API types.
pub mod management;
/// Device registry for multi-device servers.
pub mod registry;
/// Core protocol types: errors, responses, device types, parameters.
pub mod types;

/// `ConformU` test harness for conformance validation.
#[cfg(feature = "conformu")]
pub mod conformu;

/// ASCOM Camera device type.
#[cfg(feature = "camera")]
pub mod camera;
/// ASCOM `CoverCalibrator` device type.
#[cfg(feature = "cover_calibrator")]
pub mod cover_calibrator;
/// ASCOM Dome device type.
#[cfg(feature = "dome")]
pub mod dome;
/// ASCOM `FilterWheel` device type.
#[cfg(feature = "filter_wheel")]
pub mod filter_wheel;
/// ASCOM Focuser device type.
#[cfg(feature = "focuser")]
pub mod focuser;
/// ASCOM `ObservingConditions` device type.
#[cfg(feature = "observing_conditions")]
pub mod observing_conditions;
/// ASCOM Rotator device type.
#[cfg(feature = "rotator")]
pub mod rotator;
/// ASCOM `SafetyMonitor` device type.
#[cfg(feature = "safety_monitor")]
pub mod safety_monitor;
/// ASCOM Switch device type.
#[cfg(feature = "switch")]
pub mod switch;
/// ASCOM Telescope device type.
#[cfg(feature = "telescope")]
pub mod telescope;

/// Re-exports of the most commonly used types.
pub mod prelude {
    pub use crate::device::common::DeviceStateBuilder;
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
