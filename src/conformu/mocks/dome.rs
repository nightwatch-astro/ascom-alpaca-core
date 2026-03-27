use crate::device::Device;
use crate::dome::{Dome, ShutterState};
use crate::types::{AlpacaResult, DeviceType};

pub struct MockDome;

impl Device for MockDome {
    fn static_name(&self) -> &str { "Mock Dome" }
    fn unique_id(&self) -> &str { "mock-dome-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Dome }
    fn connected(&self) -> AlpacaResult<bool> { Ok(true) }
    fn set_connected(&self, _: bool) -> AlpacaResult<()> { Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock Dome".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(2) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Dome".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl Dome for MockDome {
    fn altitude(&self) -> AlpacaResult<f64> { Ok(90.0) }
    fn azimuth(&self) -> AlpacaResult<f64> { Ok(180.0) }
    fn at_home(&self) -> AlpacaResult<bool> { Ok(true) }
    fn at_park(&self) -> AlpacaResult<bool> { Ok(false) }
    fn shutter_status(&self) -> AlpacaResult<ShutterState> { Ok(ShutterState::Closed) }
    fn slaved(&self) -> AlpacaResult<bool> { Ok(false) }
    fn slewing(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_find_home(&self) -> AlpacaResult<bool> { Ok(true) }
    fn can_park(&self) -> AlpacaResult<bool> { Ok(true) }
    fn can_set_altitude(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_set_azimuth(&self) -> AlpacaResult<bool> { Ok(true) }
    fn can_set_park(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_set_shutter(&self) -> AlpacaResult<bool> { Ok(true) }
    fn can_slave(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_sync_azimuth(&self) -> AlpacaResult<bool> { Ok(false) }
}
