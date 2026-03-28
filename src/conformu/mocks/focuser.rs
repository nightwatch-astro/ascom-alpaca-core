use std::sync::Mutex;
use std::time::Instant;

use crate::focuser::Focuser;
use crate::types::{AlpacaResult, DeviceType};

pub struct MockFocuser {
    connected: Mutex<bool>,
    position: Mutex<i32>,
    target: Mutex<i32>,
    move_start: Mutex<Option<Instant>>,
    temp_comp: Mutex<bool>,
}

impl Default for MockFocuser {
    fn default() -> Self {
        Self::new()
    }
}

impl MockFocuser {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
            position: Mutex::new(25000),
            target: Mutex::new(25000),
            move_start: Mutex::new(None),
            temp_comp: Mutex::new(false),
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

impl_mock_device!(MockFocuser,
    name: "Mock Focuser",
    unique_id: "mock-foc-001",
    device_type: DeviceType::Focuser,
    interface_version: 4,
    device_state: |self_: &MockFocuser| {
        use crate::device::common::DeviceStateBuilder;
        self_.check_move_complete();
        Ok(DeviceStateBuilder::new()
            .add("IsMoving", self_.move_start.lock().unwrap().is_some())
            .add("Position", *self_.position.lock().unwrap())
            .add("Temperature", 20.0)
            .build())
    }
);

impl Focuser for MockFocuser {
    fn absolute(&self) -> AlpacaResult<bool> {
        Ok(true)
    }

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

    fn max_increment(&self) -> AlpacaResult<i32> {
        Ok(50000)
    }
    fn max_step(&self) -> AlpacaResult<i32> {
        Ok(50000)
    }

    fn position(&self) -> AlpacaResult<i32> {
        self.check_move_complete();
        Ok(*self.position.lock().unwrap())
    }

    fn step_size(&self) -> AlpacaResult<f64> {
        Ok(1.0)
    }
    fn temp_comp(&self) -> AlpacaResult<bool> {
        Ok(*self.temp_comp.lock().unwrap())
    }
    fn temp_comp_available(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn temperature(&self) -> AlpacaResult<f64> {
        Ok(20.0)
    }

    fn set_temp_comp(&self, enabled: bool) -> AlpacaResult<()> {
        *self.temp_comp.lock().unwrap() = enabled;
        Ok(())
    }

    fn halt(&self) -> AlpacaResult<()> {
        // Stop movement, keep current position
        self.check_move_complete();
        *self.move_start.lock().unwrap() = None;
        Ok(())
    }

    fn r#move(&self, position: i32) -> AlpacaResult<()> {
        // ASCOM spec: clamp to valid range, don't throw
        let clamped = position.clamp(0, 50000);
        *self.target.lock().unwrap() = clamped;
        *self.move_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }
}
