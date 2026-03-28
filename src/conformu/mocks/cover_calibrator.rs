use std::sync::Mutex;

use crate::cover_calibrator::{CalibratorState, CoverCalibrator, CoverState};
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

pub struct MockCoverCalibrator {
    connected: Mutex<bool>,
    cover_state: Mutex<CoverState>,
    calibrator_state: Mutex<CalibratorState>,
    brightness: Mutex<i32>,
}

impl Default for MockCoverCalibrator {
    fn default() -> Self {
        Self::new()
    }
}

impl MockCoverCalibrator {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
            cover_state: Mutex::new(CoverState::Closed),
            calibrator_state: Mutex::new(CalibratorState::Off),
            brightness: Mutex::new(0),
        }
    }
}

impl_mock_device!(MockCoverCalibrator,
    name: "Mock CoverCalibrator",
    unique_id: "mock-cc-001",
    device_type: DeviceType::CoverCalibrator,
    interface_version: 2,
    device_state: |self_: &MockCoverCalibrator| {
        use crate::device::common::DeviceStateBuilder;
        Ok(DeviceStateBuilder::new()
            .add("Brightness", *self_.brightness.lock().unwrap())
            .add("CalibratorState", *self_.calibrator_state.lock().unwrap())
            .add("CoverState", *self_.cover_state.lock().unwrap())
            .add("CalibratorChanging", false)
            .add("CoverMoving", false)
            .build())
    }
);

impl CoverCalibrator for MockCoverCalibrator {
    fn brightness(&self) -> AlpacaResult<i32> {
        Ok(*self.brightness.lock().unwrap())
    }

    fn max_brightness(&self) -> AlpacaResult<i32> {
        Ok(100)
    }

    fn calibrator_state(&self) -> AlpacaResult<CalibratorState> {
        Ok(*self.calibrator_state.lock().unwrap())
    }

    fn cover_state(&self) -> AlpacaResult<CoverState> {
        Ok(*self.cover_state.lock().unwrap())
    }

    fn calibrator_on(&self, brightness: i32) -> AlpacaResult<()> {
        if !(0..=100).contains(&brightness) {
            return Err(AlpacaError::InvalidValue(format!(
                "Brightness {brightness} out of range 0-100"
            )));
        }
        *self.calibrator_state.lock().unwrap() = CalibratorState::Ready;
        *self.brightness.lock().unwrap() = brightness;
        Ok(())
    }

    fn calibrator_off(&self) -> AlpacaResult<()> {
        *self.calibrator_state.lock().unwrap() = CalibratorState::Off;
        *self.brightness.lock().unwrap() = 0;
        Ok(())
    }

    fn open_cover(&self) -> AlpacaResult<()> {
        *self.cover_state.lock().unwrap() = CoverState::Open;
        Ok(())
    }

    fn close_cover(&self) -> AlpacaResult<()> {
        *self.cover_state.lock().unwrap() = CoverState::Closed;
        Ok(())
    }

    fn halt_cover(&self) -> AlpacaResult<()> {
        // Cover operates synchronously — ConformU requires NotImplemented in this case
        Err(AlpacaError::NotImplemented(
            "halt_cover not supported — cover operates synchronously".into(),
        ))
    }

    fn calibrator_changing(&self) -> AlpacaResult<bool> {
        Ok(false)
    }
    fn cover_moving(&self) -> AlpacaResult<bool> {
        Ok(false)
    }
}
