use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// ASCOM SafetyMonitor device trait.
///
/// A generic trigger for unsafe conditions. Not limited to weather — any
/// condition that should halt imaging operations: wind, rain, cloud cover,
/// door open, power failure, equipment malfunction, dew heater offline,
/// dead man's switch timeout, or any custom safety logic.
///
/// Imaging applications (NINA, SGP, Voyager) poll `is_safe()` and will
/// abort sequences when it returns `false`.
///
/// # Example
///
/// ```rust
/// use ascom_alpaca_core::prelude::*;
/// # struct MySafety { wind_ok: bool, power_ok: bool }
/// # impl Device for MySafety {
/// #     fn static_name(&self) -> &str { "Safety" }
/// #     fn unique_id(&self) -> &str { "s-001" }
/// #     fn device_type(&self) -> DeviceType { DeviceType::SafetyMonitor }
/// # }
///
/// impl SafetyMonitor for MySafety {
///     fn is_safe(&self) -> AlpacaResult<bool> {
///         Ok(self.wind_ok && self.power_ok)
///     }
/// }
/// ```
pub trait SafetyMonitor: Device {
    /// Returns `true` if conditions are safe for continued operation,
    /// `false` if the observatory should shut down or pause.
    fn is_safe(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("is_safe".into()))
    }
}
