use std::sync::Mutex;

use crate::device::Device;
use crate::focuser::Focuser;
use crate::types::{AlpacaResult, DeviceType};

pub struct MockFocuser {
    connected: Mutex<bool>,
}

impl MockFocuser {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
        }
    }
}

impl Device for MockFocuser {
    fn static_name(&self) -> &str { "Mock Focuser" }
    fn unique_id(&self) -> &str { "mock-foc-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Focuser }
    fn connected(&self) -> AlpacaResult<bool> { Ok(*self.connected.lock().unwrap()) }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> { *self.connected.lock().unwrap() = v; Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = true; Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = false; Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock Focuser".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(3) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Focuser".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl Focuser for MockFocuser {
    fn absolute(&self) -> AlpacaResult<bool> { Ok(true) }
    fn is_moving(&self) -> AlpacaResult<bool> { Ok(false) }
    fn max_increment(&self) -> AlpacaResult<i32> { Ok(50000) }
    fn max_step(&self) -> AlpacaResult<i32> { Ok(50000) }
    fn position(&self) -> AlpacaResult<i32> { Ok(25000) }
    fn step_size(&self) -> AlpacaResult<f64> { Ok(1.0) }
    fn temp_comp(&self) -> AlpacaResult<bool> { Ok(false) }
    fn temp_comp_available(&self) -> AlpacaResult<bool> { Ok(false) }
    fn temperature(&self) -> AlpacaResult<f64> { Ok(20.0) }
    fn halt(&self) -> AlpacaResult<()> { Ok(()) }
    fn r#move(&self, _position: i32) -> AlpacaResult<()> { Ok(()) }
}
