use std::fmt;

/// Protocol-level errors that appear in the Alpaca JSON response body (HTTP 200).
///
/// Error codes follow the ASCOM Alpaca specification:
/// - 0x400-0x40E: Standard ASCOM error codes
/// - 0x500-0xFFF: Driver-specific error codes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlpacaError {
    /// Method or property not implemented (0x400 / 1024)
    NotImplemented(String),
    /// Invalid value was supplied (0x401 / 1025)
    InvalidValue(String),
    /// Value has not been set (0x402 / 1026)
    ValueNotSet(String),
    /// Device is not connected (0x407 / 1031)
    NotConnected(String),
    /// Invalid operation while device is parked (0x408 / 1032)
    InvalidWhileParked(String),
    /// Invalid operation while device is slaved (0x409 / 1033)
    InvalidWhileSlaved(String),
    /// General invalid operation (0x40B / 1035)
    InvalidOperationException(String),
    /// Action command not implemented (0x40C / 1036)
    ActionNotImplemented(String),
    /// Operation was cancelled (0x40E / 1038)
    OperationCancelled(String),
    /// Driver-specific error (0x500-0xFFF / 1280-4095)
    DriverError { code: u32, message: String },
    /// Unknown error code
    Unknown(u32),
}

impl AlpacaError {
    /// Returns the ASCOM error code for this error.
    pub fn error_code(&self) -> i32 {
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
            Self::DriverError { code, .. } => *code as i32,
            Self::Unknown(code) => *code as i32,
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

    /// Creates an AlpacaError from a numeric error code and message.
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

impl fmt::Display for AlpacaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotImplemented(msg) => write!(f, "Not implemented: {msg}"),
            Self::InvalidValue(msg) => write!(f, "Invalid value: {msg}"),
            Self::ValueNotSet(msg) => write!(f, "Value not set: {msg}"),
            Self::NotConnected(msg) => write!(f, "Not connected: {msg}"),
            Self::InvalidWhileParked(msg) => write!(f, "Invalid while parked: {msg}"),
            Self::InvalidWhileSlaved(msg) => write!(f, "Invalid while slaved: {msg}"),
            Self::InvalidOperationException(msg) => write!(f, "Invalid operation: {msg}"),
            Self::ActionNotImplemented(msg) => write!(f, "Action not implemented: {msg}"),
            Self::OperationCancelled(msg) => write!(f, "Operation cancelled: {msg}"),
            Self::DriverError { code, message } => {
                write!(f, "Driver error (0x{code:X}): {message}")
            }
            Self::Unknown(code) => write!(f, "Unknown error (0x{code:X})"),
        }
    }
}

impl std::error::Error for AlpacaError {}

/// Convenience type alias for ASCOM Alpaca results.
pub type AlpacaResult<T> = Result<T, AlpacaError>;

/// Non-protocol errors for device lookup failures.
///
/// These map to HTTP status codes (e.g., 400 Bad Request), not JSON error envelopes.
#[derive(Debug, Clone, thiserror::Error)]
pub enum RegistryError {
    #[error("Device not found: {device_type} device {device_number}")]
    DeviceNotFound {
        device_type: crate::types::DeviceType,
        device_number: u32,
    },
    #[error("No devices registered for type: {0}")]
    DeviceTypeNotRegistered(crate::types::DeviceType),
}
