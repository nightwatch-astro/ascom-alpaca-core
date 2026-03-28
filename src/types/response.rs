use serde::{Deserialize, Serialize};

use super::AlpacaError;

/// Generic response envelope for all value-returning Alpaca endpoints.
///
/// Fields use PascalCase per the ASCOM Alpaca specification, with explicit
/// renames for fields ending in "ID" to avoid serde's default "Id" casing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AlpacaResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
    pub error_number: i32,
    pub error_message: String,
    #[serde(rename = "ClientTransactionID")]
    pub client_transaction_id: u32,
    #[serde(rename = "ServerTransactionID")]
    pub server_transaction_id: u32,
}

impl<T: Serialize> AlpacaResponse<T> {
    /// Creates a successful response with the given value.
    pub fn ok(value: T) -> Self {
        Self {
            value: Some(value),
            error_number: 0,
            error_message: String::new(),
            client_transaction_id: 0,
            server_transaction_id: 0,
        }
    }

    /// Creates an error response from an `AlpacaError`.
    pub fn from_error(error: AlpacaError) -> Self {
        Self {
            value: None,
            error_number: error.error_code(),
            error_message: error.error_message().to_string(),
            client_transaction_id: 0,
            server_transaction_id: 0,
        }
    }

    /// Creates a "not implemented" error response.
    pub fn not_implemented(method: &str) -> Self {
        Self::from_error(AlpacaError::NotImplemented(format!(
            "{method} is not implemented"
        )))
    }

    /// Sets the transaction IDs on this response.
    pub fn with_transaction(mut self, client_tx: u32, server_tx: u32) -> Self {
        self.client_transaction_id = client_tx;
        self.server_transaction_id = server_tx;
        self
    }
}

/// Response for PUT operations with no return value.
///
/// Same as `AlpacaResponse` but without the `Value` field.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MethodResponse {
    pub error_number: i32,
    pub error_message: String,
    #[serde(rename = "ClientTransactionID")]
    pub client_transaction_id: u32,
    #[serde(rename = "ServerTransactionID")]
    pub server_transaction_id: u32,
}

impl MethodResponse {
    /// Creates a successful method response (no error).
    pub fn ok() -> Self {
        Self {
            error_number: 0,
            error_message: String::new(),
            client_transaction_id: 0,
            server_transaction_id: 0,
        }
    }

    /// Creates an error method response from an `AlpacaError`.
    pub fn from_error(error: AlpacaError) -> Self {
        Self {
            error_number: error.error_code(),
            error_message: error.error_message().to_string(),
            client_transaction_id: 0,
            server_transaction_id: 0,
        }
    }

    /// Sets the transaction IDs on this response.
    pub fn with_transaction(mut self, client_tx: u32, server_tx: u32) -> Self {
        self.client_transaction_id = client_tx;
        self.server_transaction_id = server_tx;
        self
    }
}
