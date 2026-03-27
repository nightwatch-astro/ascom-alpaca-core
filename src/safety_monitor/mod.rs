use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// ASCOM SafetyMonitor device trait.
///
/// The simplest ASCOM device type — a single method indicating whether
/// conditions are safe for observatory operation.
pub trait SafetyMonitor: Device {
    /// Returns `true` if conditions are safe, `false` otherwise.
    fn is_safe(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("is_safe".into()))
    }
}
