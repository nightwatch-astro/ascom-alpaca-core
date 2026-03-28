use std::sync::Mutex;
use std::time::Instant;

use crate::rotator::Rotator;
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

pub struct MockRotator {
    connected: Mutex<bool>,
    position: Mutex<f64>,
    mechanical_position: Mutex<f64>,
    target: Mutex<f64>,
    move_start: Mutex<Option<Instant>>,
    reverse: Mutex<bool>,
}

impl Default for MockRotator {
    fn default() -> Self {
        Self::new()
    }
}

impl MockRotator {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
            position: Mutex::new(0.0),
            mechanical_position: Mutex::new(0.0),
            target: Mutex::new(0.0),
            move_start: Mutex::new(None),
            reverse: Mutex::new(false),
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

impl_mock_device!(MockRotator,
    name: "Mock Rotator",
    unique_id: "mock-rot-001",
    device_type: DeviceType::Rotator,
    interface_version: 4,
    device_state: |self_: &MockRotator| {
        use crate::device::common::DeviceStateBuilder;
        self_.check_move_complete();
        Ok(DeviceStateBuilder::new()
            .add("IsMoving", self_.move_start.lock().unwrap().is_some())
            .add("MechanicalPosition", *self_.mechanical_position.lock().unwrap())
            .add("Position", *self_.position.lock().unwrap())
            .build())
    }
);

impl Rotator for MockRotator {
    fn can_reverse(&self) -> AlpacaResult<bool> {
        Ok(true)
    }

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

    fn reverse(&self) -> AlpacaResult<bool> {
        Ok(*self.reverse.lock().unwrap())
    }
    fn set_reverse(&self, v: bool) -> AlpacaResult<()> {
        *self.reverse.lock().unwrap() = v;
        Ok(())
    }
    fn step_size(&self) -> AlpacaResult<f64> {
        Ok(1.0)
    }

    fn target_position(&self) -> AlpacaResult<f64> {
        Ok(*self.target.lock().unwrap())
    }

    fn halt(&self) -> AlpacaResult<()> {
        self.check_move_complete();
        *self.move_start.lock().unwrap() = None;
        Ok(())
    }

    fn r#move(&self, relative_position: f64) -> AlpacaResult<()> {
        self.check_move_complete();
        let current = *self.position.lock().unwrap();
        let target = (current + relative_position).rem_euclid(360.0);
        *self.target.lock().unwrap() = target;
        *self.move_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn move_absolute(&self, position: f64) -> AlpacaResult<()> {
        if !(0.0..360.0).contains(&position) {
            return Err(AlpacaError::InvalidValue(format!(
                "Position {position} out of range 0..360"
            )));
        }
        *self.target.lock().unwrap() = position;
        *self.move_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn move_mechanical(&self, position: f64) -> AlpacaResult<()> {
        if !(0.0..360.0).contains(&position) {
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
