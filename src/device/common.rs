use serde::{Deserialize, Serialize};

/// A single name-value pair in the DeviceState array (Platform 7+).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceStateItem {
    pub name: String,
    pub value: serde_json::Value,
}
