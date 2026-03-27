use crate::device::Device;
use crate::switch::Switch;
use crate::types::{AlpacaResult, DeviceType};

pub struct MockSwitch;

impl Device for MockSwitch {
    fn static_name(&self) -> &str { "Mock Switch" }
    fn unique_id(&self) -> &str { "mock-sw-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Switch }
    fn connected(&self) -> AlpacaResult<bool> { Ok(true) }
    fn set_connected(&self, _: bool) -> AlpacaResult<()> { Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock Switch".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(2) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Switch".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl Switch for MockSwitch {
    fn max_switch(&self) -> AlpacaResult<i32> { Ok(2) }
    fn can_write(&self, _id: u32) -> AlpacaResult<bool> { Ok(true) }
    fn get_switch(&self, _id: u32) -> AlpacaResult<bool> { Ok(false) }
    fn get_switch_value(&self, _id: u32) -> AlpacaResult<f64> { Ok(0.0) }
    fn get_switch_name(&self, id: u32) -> AlpacaResult<String> { Ok(format!("Switch {id}")) }
    fn get_switch_description(&self, id: u32) -> AlpacaResult<String> { Ok(format!("Mock switch channel {id}")) }
    fn min_switch_value(&self, _id: u32) -> AlpacaResult<f64> { Ok(0.0) }
    fn max_switch_value(&self, _id: u32) -> AlpacaResult<f64> { Ok(1.0) }
    fn switch_step(&self, _id: u32) -> AlpacaResult<f64> { Ok(1.0) }
    fn set_switch(&self, _id: u32, _state: bool) -> AlpacaResult<()> { Ok(()) }
    fn set_switch_value(&self, _id: u32, _value: f64) -> AlpacaResult<()> { Ok(()) }
}
