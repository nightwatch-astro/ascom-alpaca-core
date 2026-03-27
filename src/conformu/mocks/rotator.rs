use std::sync::Mutex;

use crate::device::Device;
use crate::rotator::Rotator;
use crate::types::{AlpacaResult, DeviceType};

pub struct MockRotator {
    connected: Mutex<bool>,
}

impl MockRotator {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
        }
    }
}

impl Device for MockRotator {
    fn static_name(&self) -> &str { "Mock Rotator" }
    fn unique_id(&self) -> &str { "mock-rot-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Rotator }
    fn connected(&self) -> AlpacaResult<bool> { Ok(*self.connected.lock().unwrap()) }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> { *self.connected.lock().unwrap() = v; Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = true; Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = false; Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock Rotator".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(3) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Rotator".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl Rotator for MockRotator {
    fn can_reverse(&self) -> AlpacaResult<bool> { Ok(false) }
    fn is_moving(&self) -> AlpacaResult<bool> { Ok(false) }
    fn mechanical_position(&self) -> AlpacaResult<f64> { Ok(0.0) }
    fn position(&self) -> AlpacaResult<f64> { Ok(0.0) }
    fn reverse(&self) -> AlpacaResult<bool> { Ok(false) }
    fn step_size(&self) -> AlpacaResult<f64> { Ok(1.0) }
    fn target_position(&self) -> AlpacaResult<f64> { Ok(0.0) }
    fn halt(&self) -> AlpacaResult<()> { Ok(()) }
    fn move_absolute(&self, _position: f64) -> AlpacaResult<()> { Ok(()) }
    fn move_mechanical(&self, _position: f64) -> AlpacaResult<()> { Ok(()) }
    fn sync(&self, _position: f64) -> AlpacaResult<()> { Ok(()) }
}
