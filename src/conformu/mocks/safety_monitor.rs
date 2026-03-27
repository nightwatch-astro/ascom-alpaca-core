use std::sync::Mutex;

use crate::device::Device;
use crate::safety_monitor::SafetyMonitor;
use crate::types::{AlpacaResult, DeviceType};

pub struct MockSafetyMonitor {
    connected: Mutex<bool>,
}

impl MockSafetyMonitor {
    pub fn new() -> Self {
        Self { connected: Mutex::new(false) }
    }
}

impl Device for MockSafetyMonitor {
    fn static_name(&self) -> &str { "Mock Safety Monitor" }
    fn unique_id(&self) -> &str { "mock-sm-001" }
    fn device_type(&self) -> DeviceType { DeviceType::SafetyMonitor }
    fn connected(&self) -> AlpacaResult<bool> { Ok(*self.connected.lock().unwrap()) }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> { *self.connected.lock().unwrap() = v; Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = true; Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = false; Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock SafetyMonitor for ConformU testing".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock driver".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(3) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Safety Monitor".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
    fn device_state(&self) -> AlpacaResult<Vec<crate::device::common::DeviceStateItem>> {
        use crate::device::common::DeviceStateItem;
        Ok(vec![
            DeviceStateItem { name: "IsSafe".into(), value: serde_json::json!(true) },
        ])
    }
}

impl SafetyMonitor for MockSafetyMonitor {
    fn is_safe(&self) -> AlpacaResult<bool> { Ok(true) }
}
