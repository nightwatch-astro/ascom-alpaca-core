use serde::{Deserialize, Serialize};

use crate::types::DeviceType;

/// Server metadata for the Alpaca management API.
pub struct ServerInfo {
    /// The server description metadata.
    pub description: ServerDescription,
}

impl ServerInfo {
    /// Creates a new `ServerInfo` with the given description.
    pub const fn new(description: ServerDescription) -> Self {
        Self { description }
    }

    /// Returns the API versions supported by this server.
    pub fn api_versions(&self) -> ApiVersionsResponse {
        ApiVersionsResponse { value: vec![1] }
    }
}

/// Server description for the `/management/v1/description` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerDescription {
    /// Display name of the server.
    pub server_name: String,
    /// Name of the server manufacturer.
    pub manufacturer: String,
    /// Version string of the server software.
    pub manufacturer_version: String,
    /// Physical location of the server.
    pub location: String,
}

/// Response for the `/management/apiversions` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiVersionsResponse {
    /// The list of supported API version numbers.
    pub value: Vec<u32>,
}

/// A configured device entry for the `/management/v1/configureddevices` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfiguredDevice {
    /// Display name of the device.
    pub device_name: String,
    /// The ASCOM device type.
    pub device_type: DeviceType,
    /// The zero-based device number within its type.
    pub device_number: u32,
    /// A unique identifier for this device instance.
    #[serde(rename = "UniqueID")]
    pub unique_id: String,
}
