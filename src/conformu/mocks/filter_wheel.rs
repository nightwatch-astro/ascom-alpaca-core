use std::sync::Mutex;

use crate::device::Device;
use crate::filter_wheel::FilterWheel;
use crate::types::{AlpacaResult, DeviceType};

pub struct MockFilterWheel {
    connected: Mutex<bool>,
}

impl MockFilterWheel {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
        }
    }
}

impl Device for MockFilterWheel {
    fn static_name(&self) -> &str { "Mock FilterWheel" }
    fn unique_id(&self) -> &str { "mock-fw-001" }
    fn device_type(&self) -> DeviceType { DeviceType::FilterWheel }
    fn connected(&self) -> AlpacaResult<bool> { Ok(*self.connected.lock().unwrap()) }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> { *self.connected.lock().unwrap() = v; Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = true; Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = false; Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock FilterWheel".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(2) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock FilterWheel".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl FilterWheel for MockFilterWheel {
    fn position(&self) -> AlpacaResult<i32> { Ok(0) }
    fn set_position(&self, _position: i32) -> AlpacaResult<()> { Ok(()) }
    fn names(&self) -> AlpacaResult<Vec<String>> { Ok(vec!["Red".into(), "Green".into(), "Blue".into(), "Luminance".into()]) }
    fn focus_offsets(&self) -> AlpacaResult<Vec<i32>> { Ok(vec![0, 0, 0, 0]) }
}
