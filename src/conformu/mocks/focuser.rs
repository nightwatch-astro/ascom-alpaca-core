use std::sync::Mutex;
use std::time::Instant;

use crate::device::Device;
use crate::focuser::Focuser;
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

pub struct MockFocuser {
    connected: Mutex<bool>,
    position: Mutex<i32>,
    target: Mutex<i32>,
    move_start: Mutex<Option<Instant>>,
}

impl MockFocuser {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
            position: Mutex::new(25000),
            target: Mutex::new(25000),
            move_start: Mutex::new(None),
        }
    }

    /// Check if a move has completed based on elapsed time.
    fn check_move_complete(&self) {
        let start = *self.move_start.lock().unwrap();
        if let Some(started_at) = start {
            if started_at.elapsed().as_millis() >= 100 {
                let target = *self.target.lock().unwrap();
                *self.position.lock().unwrap() = target;
                *self.move_start.lock().unwrap() = None;
            }
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
    fn device_state(&self) -> AlpacaResult<Vec<crate::device::common::DeviceStateItem>> { Ok(vec![]) }
}

impl Focuser for MockFocuser {
    fn absolute(&self) -> AlpacaResult<bool> { Ok(true) }

    fn is_moving(&self) -> AlpacaResult<bool> {
        let start = *self.move_start.lock().unwrap();
        if let Some(started_at) = start {
            if started_at.elapsed().as_millis() < 100 {
                return Ok(true);
            }
            // Move complete — update position
            let target = *self.target.lock().unwrap();
            *self.position.lock().unwrap() = target;
            *self.move_start.lock().unwrap() = None;
        }
        Ok(false)
    }

    fn max_increment(&self) -> AlpacaResult<i32> { Ok(50000) }
    fn max_step(&self) -> AlpacaResult<i32> { Ok(50000) }

    fn position(&self) -> AlpacaResult<i32> {
        self.check_move_complete();
        Ok(*self.position.lock().unwrap())
    }

    fn step_size(&self) -> AlpacaResult<f64> { Ok(1.0) }
    fn temp_comp(&self) -> AlpacaResult<bool> { Ok(false) }
    fn temp_comp_available(&self) -> AlpacaResult<bool> { Ok(false) }
    fn temperature(&self) -> AlpacaResult<f64> { Ok(20.0) }

    fn set_temp_comp(&self, _enabled: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("Temperature compensation is not available".into()))
    }

    fn halt(&self) -> AlpacaResult<()> {
        // Stop movement, keep current position
        self.check_move_complete();
        *self.move_start.lock().unwrap() = None;
        Ok(())
    }

    fn r#move(&self, position: i32) -> AlpacaResult<()> {
        if position < 0 || position > 50000 {
            return Err(AlpacaError::InvalidValue(format!(
                "Position {position} out of range 0-50000"
            )));
        }
        *self.target.lock().unwrap() = position;
        *self.move_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }
}
