use std::sync::Mutex;

use crate::safety_monitor::SafetyMonitor;
use crate::types::{AlpacaResult, DeviceType};

pub struct MockSafetyMonitor {
    connected: Mutex<bool>,
}

impl Default for MockSafetyMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl MockSafetyMonitor {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
        }
    }
}

impl_mock_device!(MockSafetyMonitor,
    name: "Mock Safety Monitor",
    unique_id: "mock-sm-001",
    device_type: DeviceType::SafetyMonitor,
    interface_version: 3,
    device_state: |_self: &MockSafetyMonitor| {
        use crate::device::common::DeviceStateBuilder;
        Ok(DeviceStateBuilder::new().add("IsSafe", true).build())
    }
);

impl SafetyMonitor for MockSafetyMonitor {
    fn is_safe(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
}
