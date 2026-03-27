use serde::{Deserialize, Serialize};

use crate::types::DeviceType;

/// Server metadata for the Alpaca management API.
pub struct ServerInfo {
    pub description: ServerDescription,
}

impl ServerInfo {
    pub fn new(description: ServerDescription) -> Self {
        Self { description }
    }

    /// Returns the API versions supported by this server.
    pub fn api_versions(&self) -> ApiVersionsResponse {
        ApiVersionsResponse {
            value: vec![1],
        }
    }
}

/// Server description for the `/management/v1/description` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerDescription {
    pub server_name: String,
    pub manufacturer: String,
    pub manufacturer_version: String,
    pub location: String,
}

/// Response for the `/management/apiversions` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiVersionsResponse {
    pub value: Vec<u32>,
}

/// A configured device entry for the `/management/v1/configureddevices` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfiguredDevice {
    pub device_name: String,
    pub device_type: DeviceType,
    pub device_number: u32,
    #[serde(rename = "UniqueID")]
    pub unique_id: String,
}
