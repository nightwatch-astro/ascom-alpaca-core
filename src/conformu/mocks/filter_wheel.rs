use std::sync::Mutex;
use std::time::Instant;

use crate::device::Device;
use crate::filter_wheel::FilterWheel;
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

pub struct MockFilterWheel {
    connected: Mutex<bool>,
    position: Mutex<i32>,
    target_position: Mutex<i32>,
    move_start: Mutex<Option<Instant>>,
}

impl Default for MockFilterWheel {
    fn default() -> Self {
        Self::new()
    }
}

impl MockFilterWheel {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
            position: Mutex::new(0),
            target_position: Mutex::new(0),
            move_start: Mutex::new(None),
        }
    }
}

impl Device for MockFilterWheel {
    fn static_name(&self) -> &str {
        "Mock FilterWheel"
    }
    fn unique_id(&self) -> &str {
        "mock-fw-001"
    }
    fn device_type(&self) -> DeviceType {
        DeviceType::FilterWheel
    }
    fn connected(&self) -> AlpacaResult<bool> {
        Ok(*self.connected.lock().unwrap())
    }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> {
        *self.connected.lock().unwrap() = v;
        Ok(())
    }
    fn connecting(&self) -> AlpacaResult<bool> {
        Ok(false)
    }
    fn connect(&self) -> AlpacaResult<()> {
        *self.connected.lock().unwrap() = true;
        Ok(())
    }
    fn disconnect(&self) -> AlpacaResult<()> {
        *self.connected.lock().unwrap() = false;
        Ok(())
    }
    fn description(&self) -> AlpacaResult<String> {
        Ok("Mock FilterWheel".into())
    }
    fn driver_info(&self) -> AlpacaResult<String> {
        Ok("ascom-alpaca-core mock".into())
    }
    fn driver_version(&self) -> AlpacaResult<String> {
        Ok(env!("CARGO_PKG_VERSION").into())
    }
    fn interface_version(&self) -> AlpacaResult<i32> {
        Ok(3)
    }
    fn name(&self) -> AlpacaResult<String> {
        Ok("Mock FilterWheel".into())
    }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> {
        Ok(vec![])
    }
    fn device_state(&self) -> AlpacaResult<Vec<crate::device::common::DeviceStateItem>> {
        use crate::device::common::DeviceStateItem;
        let pos = self.position().unwrap_or(-1);
        Ok(vec![DeviceStateItem {
            name: "Position".into(),
            value: serde_json::json!(pos),
        }])
    }
}

impl FilterWheel for MockFilterWheel {
    fn position(&self) -> AlpacaResult<i32> {
        let start = *self.move_start.lock().unwrap();
        if let Some(started_at) = start {
            if started_at.elapsed().as_millis() < 200 {
                return Ok(-1);
            }
            // Move complete
            *self.move_start.lock().unwrap() = None;
            let target = *self.target_position.lock().unwrap();
            *self.position.lock().unwrap() = target;
        }
        Ok(*self.position.lock().unwrap())
    }

    fn set_position(&self, position: i32) -> AlpacaResult<()> {
        let num_filters = 4; // Red, Green, Blue, Luminance
        if position < 0 || position >= num_filters {
            return Err(AlpacaError::InvalidValue(format!(
                "Position {position} out of range 0-{}",
                num_filters - 1
            )));
        }
        *self.target_position.lock().unwrap() = position;
        *self.move_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn names(&self) -> AlpacaResult<Vec<String>> {
        Ok(vec![
            "Red".into(),
            "Green".into(),
            "Blue".into(),
            "Luminance".into(),
        ])
    }

    fn focus_offsets(&self) -> AlpacaResult<Vec<i32>> {
        Ok(vec![0, 0, 0, 0])
    }
}
