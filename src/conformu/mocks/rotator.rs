use std::sync::Mutex;
use std::time::Instant;

use crate::device::Device;
use crate::rotator::Rotator;
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

pub struct MockRotator {
    connected: Mutex<bool>,
    position: Mutex<f64>,
    mechanical_position: Mutex<f64>,
    target: Mutex<f64>,
    move_start: Mutex<Option<Instant>>,
}

impl MockRotator {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
            position: Mutex::new(0.0),
            mechanical_position: Mutex::new(0.0),
            target: Mutex::new(0.0),
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
                *self.mechanical_position.lock().unwrap() = target;
                *self.move_start.lock().unwrap() = None;
            }
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
    fn device_state(&self) -> AlpacaResult<Vec<crate::device::common::DeviceStateItem>> { Ok(vec![]) }
}

impl Rotator for MockRotator {
    fn can_reverse(&self) -> AlpacaResult<bool> { Ok(false) }

    fn is_moving(&self) -> AlpacaResult<bool> {
        let start = *self.move_start.lock().unwrap();
        if let Some(started_at) = start {
            if started_at.elapsed().as_millis() < 100 {
                return Ok(true);
            }
            // Move complete — update positions
            let target = *self.target.lock().unwrap();
            *self.position.lock().unwrap() = target;
            *self.mechanical_position.lock().unwrap() = target;
            *self.move_start.lock().unwrap() = None;
        }
        Ok(false)
    }

    fn mechanical_position(&self) -> AlpacaResult<f64> {
        self.check_move_complete();
        Ok(*self.mechanical_position.lock().unwrap())
    }

    fn position(&self) -> AlpacaResult<f64> {
        self.check_move_complete();
        Ok(*self.position.lock().unwrap())
    }

    fn reverse(&self) -> AlpacaResult<bool> { Ok(false) }
    fn step_size(&self) -> AlpacaResult<f64> { Ok(1.0) }

    fn target_position(&self) -> AlpacaResult<f64> {
        Ok(*self.target.lock().unwrap())
    }

    fn halt(&self) -> AlpacaResult<()> {
        self.check_move_complete();
        *self.move_start.lock().unwrap() = None;
        Ok(())
    }

    fn move_absolute(&self, position: f64) -> AlpacaResult<()> {
        if position < 0.0 || position >= 360.0 {
            return Err(AlpacaError::InvalidValue(format!(
                "Position {position} out of range 0..360"
            )));
        }
        *self.target.lock().unwrap() = position;
        *self.move_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn move_mechanical(&self, position: f64) -> AlpacaResult<()> {
        if position < 0.0 || position >= 360.0 {
            return Err(AlpacaError::InvalidValue(format!(
                "Position {position} out of range 0..360"
            )));
        }
        *self.target.lock().unwrap() = position;
        *self.move_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn sync(&self, position: f64) -> AlpacaResult<()> {
        *self.position.lock().unwrap() = position;
        *self.mechanical_position.lock().unwrap() = position;
        *self.target.lock().unwrap() = position;
        Ok(())
    }
}
