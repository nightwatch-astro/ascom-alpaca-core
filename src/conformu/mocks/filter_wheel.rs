use std::sync::Mutex;
use std::time::Instant;

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

impl_mock_device!(MockFilterWheel,
    name: "Mock FilterWheel",
    unique_id: "mock-fw-001",
    device_type: DeviceType::FilterWheel,
    interface_version: 3,
    device_state: |self_: &MockFilterWheel| {
        use crate::device::common::DeviceStateBuilder;
        let pos = self_.position().unwrap_or(-1);
        Ok(DeviceStateBuilder::new().add("Position", pos).build())
    }
);

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
