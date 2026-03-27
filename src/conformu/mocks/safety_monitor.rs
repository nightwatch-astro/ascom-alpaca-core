use crate::device::Device;
use crate::safety_monitor::SafetyMonitor;
use crate::types::{AlpacaResult, DeviceType};

pub struct MockSafetyMonitor;

impl Device for MockSafetyMonitor {
    fn static_name(&self) -> &str { "Mock Safety Monitor" }
    fn unique_id(&self) -> &str { "mock-sm-001" }
    fn device_type(&self) -> DeviceType { DeviceType::SafetyMonitor }
    fn connected(&self) -> AlpacaResult<bool> { Ok(true) }
    fn set_connected(&self, _: bool) -> AlpacaResult<()> { Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock SafetyMonitor for ConformU testing".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock driver".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(1) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Safety Monitor".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl SafetyMonitor for MockSafetyMonitor {
    fn is_safe(&self) -> AlpacaResult<bool> { Ok(true) }
}
