use serde::{Deserialize, Serialize};

/// A single name-value pair in the `DeviceState` array (Platform 7+).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceStateItem {
    /// The state property name (e.g., `IsSafe`, `Temperature`).
    pub name: String,
    /// The state property value.
    pub value: serde_json::Value,
}

/// Builder for constructing a `Vec<DeviceStateItem>` with minimal boilerplate.
///
/// # Example
///
/// ```rust
/// use ascom_alpaca_core::device::common::DeviceStateBuilder;
///
/// let state = DeviceStateBuilder::new()
///     .add("IsSafe", true)
///     .add("Temperature", 20.5)
///     .add("SensorName", "Mock Sensor")
///     .build();
///
/// assert_eq!(state.len(), 3);
/// assert_eq!(state[0].name, "IsSafe");
/// ```
pub struct DeviceStateBuilder {
    items: Vec<DeviceStateItem>,
}

impl DeviceStateBuilder {
    /// Creates an empty builder.
    pub const fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Adds a name-value pair. Accepts any type that implements `Serialize`.
    ///
    /// If serialization fails (which should not happen for standard types),
    /// the value is stored as `serde_json::Value::Null`.
    pub fn add(mut self, name: &str, value: impl Serialize) -> Self {
        self.items.push(DeviceStateItem {
            name: name.into(),
            value: serde_json::to_value(value).unwrap_or_default(),
        });
        self
    }

    /// Consumes the builder and returns the collected state items.
    pub fn build(self) -> Vec<DeviceStateItem> {
        self.items
    }
}

impl Default for DeviceStateBuilder {
    fn default() -> Self {
        Self::new()
    }
}
