use std::collections::HashMap;
use std::sync::Mutex;

use crate::device::Device;
use crate::switch::Switch;
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

const NUM_SWITCHES: u32 = 2;

pub struct MockSwitch {
    connected: Mutex<bool>,
    values: Mutex<HashMap<u32, f64>>,
}

impl MockSwitch {
    pub fn new() -> Self {
        let mut values = HashMap::new();
        for i in 0..NUM_SWITCHES {
            values.insert(i, 0.0);
        }
        Self {
            connected: Mutex::new(false),
            values: Mutex::new(values),
        }
    }

    fn validate_id(id: u32) -> AlpacaResult<()> {
        if id >= NUM_SWITCHES {
            Err(AlpacaError::InvalidValue(format!("Switch ID {id} out of range 0-{}", NUM_SWITCHES - 1)))
        } else {
            Ok(())
        }
    }
}

impl Device for MockSwitch {
    fn static_name(&self) -> &str { "Mock Switch" }
    fn unique_id(&self) -> &str { "mock-sw-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Switch }
    fn connected(&self) -> AlpacaResult<bool> { Ok(*self.connected.lock().unwrap()) }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> { *self.connected.lock().unwrap() = v; Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = true; Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = false; Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock Switch".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(2) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Switch".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl Switch for MockSwitch {
    fn max_switch(&self) -> AlpacaResult<i32> { Ok(NUM_SWITCHES as i32) }

    fn can_write(&self, id: u32) -> AlpacaResult<bool> {
        Self::validate_id(id)?;
        Ok(true)
    }

    fn get_switch(&self, id: u32) -> AlpacaResult<bool> {
        Self::validate_id(id)?;
        let values = self.values.lock().unwrap();
        Ok(values.get(&id).copied().unwrap_or(0.0) >= 0.5)
    }

    fn set_switch(&self, id: u32, state: bool) -> AlpacaResult<()> {
        Self::validate_id(id)?;
        let mut values = self.values.lock().unwrap();
        values.insert(id, if state { 1.0 } else { 0.0 });
        Ok(())
    }

    fn get_switch_value(&self, id: u32) -> AlpacaResult<f64> {
        Self::validate_id(id)?;
        let values = self.values.lock().unwrap();
        Ok(values.get(&id).copied().unwrap_or(0.0))
    }

    fn set_switch_value(&self, id: u32, value: f64) -> AlpacaResult<()> {
        Self::validate_id(id)?;
        if value < 0.0 || value > 1.0 {
            return Err(AlpacaError::InvalidValue(format!("Value {value} out of range 0.0-1.0")));
        }
        let mut values = self.values.lock().unwrap();
        values.insert(id, value);
        Ok(())
    }

    fn get_switch_name(&self, id: u32) -> AlpacaResult<String> {
        Self::validate_id(id)?;
        Ok(format!("Switch {id}"))
    }

    fn get_switch_description(&self, id: u32) -> AlpacaResult<String> {
        Self::validate_id(id)?;
        Ok(format!("Mock switch channel {id}"))
    }

    fn min_switch_value(&self, id: u32) -> AlpacaResult<f64> {
        Self::validate_id(id)?;
        Ok(0.0)
    }

    fn max_switch_value(&self, id: u32) -> AlpacaResult<f64> {
        Self::validate_id(id)?;
        Ok(1.0)
    }

    fn switch_step(&self, id: u32) -> AlpacaResult<f64> {
        Self::validate_id(id)?;
        Ok(1.0)
    }
}
