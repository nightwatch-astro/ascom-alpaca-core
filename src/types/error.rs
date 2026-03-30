/// Protocol-level errors that appear in the Alpaca JSON response body (HTTP 200).
///
/// Error codes follow the ASCOM Alpaca specification:
/// - 0x400-0x40E: Standard ASCOM error codes
/// - 0x500-0xFFF: Driver-specific error codes
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AlpacaError {
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    #[error("Value not set: {0}")]
    ValueNotSet(String),
    #[error("Not connected: {0}")]
    NotConnected(String),
    #[error("Invalid while parked: {0}")]
    InvalidWhileParked(String),
    #[error("Invalid while slaved: {0}")]
    InvalidWhileSlaved(String),
    #[error("Invalid operation: {0}")]
    InvalidOperationException(String),
    #[error("Action not implemented: {0}")]
    ActionNotImplemented(String),
    #[error("Operation cancelled: {0}")]
    OperationCancelled(String),
    #[error("Driver error (0x{code:X}): {message}")]
    DriverError { code: u32, message: String },
    #[error("Unknown error (0x{0:X})")]
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
