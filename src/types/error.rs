/// Protocol-level errors that appear in the Alpaca JSON response body (HTTP 200).
///
/// Error codes follow the ASCOM Alpaca specification:
/// - `0x400`--`0x40E`: Standard ASCOM error codes
/// - `0x500`--`0xFFF`: Driver-specific error codes
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AlpacaError {
    /// Method or property not implemented (0x400 / 1024)
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    /// Value supplied is invalid (0x401 / 1025)
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    /// Value has not been set (0x402 / 1026)
    #[error("Value not set: {0}")]
    ValueNotSet(String),
    /// Device is not connected (0x407 / 1031)
    #[error("Not connected: {0}")]
    NotConnected(String),
    /// Invalid operation while device is parked (0x408 / 1032)
    #[error("Invalid while parked: {0}")]
    InvalidWhileParked(String),
    /// Invalid operation while device is slaved (0x409 / 1033)
    #[error("Invalid while slaved: {0}")]
    InvalidWhileSlaved(String),
    /// General invalid operation (0x40B / 1035)
    #[error("Invalid operation: {0}")]
    InvalidOperationException(String),
    /// Action command not implemented (0x40C / 1036)
    #[error("Action not implemented: {0}")]
    ActionNotImplemented(String),
    /// Operation was cancelled (0x40E / 1038)
    #[error("Operation cancelled: {0}")]
    OperationCancelled(String),
    /// Driver-specific error (0x500-0xFFF / 1280-4095)
    #[error("Driver error (0x{code:X}): {message}")]
    DriverError {
        /// The driver-specific error code (0x500--0xFFF).
        code: u32,
        /// Human-readable error description.
        message: String,
    },
    /// Unknown error code
    #[error("Unknown error (0x{0:X})")]
    Unknown(u32),
}

impl AlpacaError {
    /// Returns the ASCOM error code for this error.
    ///
    /// Driver-specific codes (`0x500`--`0xFFF`) and unknown codes are
    /// truncated to `i32`. Values above `i32::MAX` are not expected in
    /// practice per the ASCOM specification.
    #[allow(clippy::cast_possible_wrap)]
    pub const fn error_code(&self) -> i32 {
        match self {
            Self::NotImplemented(_) => 0x400,
            Self::InvalidValue(_) => 0x401,
            Self::ValueNotSet(_) => 0x402,
            Self::NotConnected(_) => 0x407,
            Self::InvalidWhileParked(_) => 0x408,
            Self::InvalidWhileSlaved(_) => 0x409,
            Self::InvalidOperationException(_) => 0x40B,
            Self::ActionNotImplemented(_) => 0x40C,
            Self::OperationCancelled(_) => 0x40E,
            Self::DriverError { code, .. } | Self::Unknown(code) => *code as i32,
        }
    }

    /// Returns the error message.
    pub fn error_message(&self) -> &str {
        match self {
            Self::NotImplemented(msg)
            | Self::InvalidValue(msg)
            | Self::ValueNotSet(msg)
            | Self::NotConnected(msg)
            | Self::InvalidWhileParked(msg)
            | Self::InvalidWhileSlaved(msg)
            | Self::InvalidOperationException(msg)
            | Self::ActionNotImplemented(msg)
            | Self::OperationCancelled(msg)
            | Self::DriverError { message: msg, .. } => msg,
            Self::Unknown(_) => "Unknown error",
        }
    }

    /// Creates an `AlpacaError` from a numeric error code and message.
    pub fn from_code(code: u32, message: String) -> Self {
        match code {
            0x400 => Self::NotImplemented(message),
            0x401 => Self::InvalidValue(message),
            0x402 => Self::ValueNotSet(message),
            0x407 => Self::NotConnected(message),
            0x408 => Self::InvalidWhileParked(message),
            0x409 => Self::InvalidWhileSlaved(message),
            0x40B => Self::InvalidOperationException(message),
            0x40C => Self::ActionNotImplemented(message),
            0x40E => Self::OperationCancelled(message),
            0x500..=0xFFF => Self::DriverError { code, message },
            _ => Self::Unknown(code),
        }
    }
}

/// Convenience type alias for ASCOM Alpaca results.
pub type AlpacaResult<T> = Result<T, AlpacaError>;

/// Non-protocol errors for device lookup failures.
///
/// These map to HTTP status codes (e.g., 400 Bad Request), not JSON error envelopes.
#[derive(Debug, Clone, thiserror::Error)]
pub enum RegistryError {
    /// The requested device number was not found in the registry.
    #[error("Device not found: {device_type} device {device_number}")]
    DeviceNotFound {
        /// The type of device that was requested.
        device_type: crate::types::DeviceType,
        /// The device number that was requested.
        device_number: u32,
    },
    /// No devices of the requested type are registered.
    #[error("No devices registered for type: {0}")]
    DeviceTypeNotRegistered(crate::types::DeviceType),
}
