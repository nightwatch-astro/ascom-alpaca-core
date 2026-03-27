use std::sync::Mutex;

use crate::device::Device;
use crate::dome::{Dome, ShutterState};
use crate::types::{AlpacaResult, DeviceType};

pub struct MockDome {
    connected: Mutex<bool>,
    shutter: Mutex<ShutterState>,
    at_home: Mutex<bool>,
    at_park: Mutex<bool>,
    azimuth: Mutex<f64>,
}

impl MockDome {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
            shutter: Mutex::new(ShutterState::Closed),
            at_home: Mutex::new(true),
            at_park: Mutex::new(false),
            azimuth: Mutex::new(180.0),
        }
    }
}

impl Device for MockDome {
    fn static_name(&self) -> &str { "Mock Dome" }
    fn unique_id(&self) -> &str { "mock-dome-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Dome }
    fn connected(&self) -> AlpacaResult<bool> { Ok(*self.connected.lock().unwrap()) }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> { *self.connected.lock().unwrap() = v; Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = true; Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = false; Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock Dome".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(2) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Dome".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
    fn device_state(&self) -> AlpacaResult<Vec<crate::device::common::DeviceStateItem>> { Ok(vec![]) }
}

impl Dome for MockDome {
    fn altitude(&self) -> AlpacaResult<f64> { Ok(90.0) }
    fn azimuth(&self) -> AlpacaResult<f64> { Ok(*self.azimuth.lock().unwrap()) }
    fn at_home(&self) -> AlpacaResult<bool> { Ok(*self.at_home.lock().unwrap()) }
    fn at_park(&self) -> AlpacaResult<bool> { Ok(*self.at_park.lock().unwrap()) }
    fn shutter_status(&self) -> AlpacaResult<ShutterState> { Ok(*self.shutter.lock().unwrap()) }
    fn slaved(&self) -> AlpacaResult<bool> { Ok(false) }
    fn slewing(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_find_home(&self) -> AlpacaResult<bool> { Ok(true) }
    fn can_park(&self) -> AlpacaResult<bool> { Ok(true) }
    fn can_set_altitude(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_set_azimuth(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_set_park(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_set_shutter(&self) -> AlpacaResult<bool> { Ok(true) }
    fn can_slave(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_sync_azimuth(&self) -> AlpacaResult<bool> { Ok(false) }

    fn open_shutter(&self) -> AlpacaResult<()> {
        *self.shutter.lock().unwrap() = ShutterState::Open;
        Ok(())
    }

    fn close_shutter(&self) -> AlpacaResult<()> {
        *self.shutter.lock().unwrap() = ShutterState::Closed;
        Ok(())
    }

    fn park(&self) -> AlpacaResult<()> {
        *self.at_park.lock().unwrap() = true;
        Ok(())
    }

    fn find_home(&self) -> AlpacaResult<()> {
        *self.at_home.lock().unwrap() = true;
        Ok(())
    }

    fn abort_slew(&self) -> AlpacaResult<()> { Ok(()) }
}
