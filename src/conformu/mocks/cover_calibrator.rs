use std::sync::Mutex;

use crate::cover_calibrator::{CalibratorState, CoverCalibrator, CoverState};
use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

pub struct MockCoverCalibrator {
    connected: Mutex<bool>,
    cover_state: Mutex<CoverState>,
    calibrator_state: Mutex<CalibratorState>,
    brightness: Mutex<i32>,
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

impl Device for MockCoverCalibrator {
    fn static_name(&self) -> &str { "Mock CoverCalibrator" }
    fn unique_id(&self) -> &str { "mock-cc-001" }
    fn device_type(&self) -> DeviceType { DeviceType::CoverCalibrator }
    fn connected(&self) -> AlpacaResult<bool> { Ok(*self.connected.lock().unwrap()) }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> { *self.connected.lock().unwrap() = v; Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = true; Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = false; Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock CoverCalibrator".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(2) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock CoverCalibrator".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
    fn device_state(&self) -> AlpacaResult<Vec<crate::device::common::DeviceStateItem>> { Ok(vec![]) }
}

impl CoverCalibrator for MockCoverCalibrator {
    fn brightness(&self) -> AlpacaResult<i32> {
        Ok(*self.brightness.lock().unwrap())
    }

    fn max_brightness(&self) -> AlpacaResult<i32> { Ok(100) }

    fn calibrator_state(&self) -> AlpacaResult<CalibratorState> {
        Ok(*self.calibrator_state.lock().unwrap())
    }

    fn cover_state(&self) -> AlpacaResult<CoverState> {
        Ok(*self.cover_state.lock().unwrap())
    }

    fn calibrator_on(&self, brightness: i32) -> AlpacaResult<()> {
        if brightness < 0 || brightness > 100 {
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
        Err(AlpacaError::NotImplemented("halt_cover not supported — cover operates synchronously".into()))
    }

    fn calibrator_changing(&self) -> AlpacaResult<bool> { Ok(false) }
    fn cover_moving(&self) -> AlpacaResult<bool> { Ok(false) }
}
