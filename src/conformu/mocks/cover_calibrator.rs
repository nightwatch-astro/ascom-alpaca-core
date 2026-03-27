use crate::cover_calibrator::{CalibratorState, CoverCalibrator, CoverState};
use crate::device::Device;
use crate::types::{AlpacaResult, DeviceType};

pub struct MockCoverCalibrator;

impl Device for MockCoverCalibrator {
    fn static_name(&self) -> &str { "Mock CoverCalibrator" }
    fn unique_id(&self) -> &str { "mock-cc-001" }
    fn device_type(&self) -> DeviceType { DeviceType::CoverCalibrator }
    fn connected(&self) -> AlpacaResult<bool> { Ok(true) }
    fn set_connected(&self, _: bool) -> AlpacaResult<()> { Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock CoverCalibrator".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(1) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock CoverCalibrator".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl CoverCalibrator for MockCoverCalibrator {
    fn brightness(&self) -> AlpacaResult<i32> { Ok(0) }
    fn max_brightness(&self) -> AlpacaResult<i32> { Ok(100) }
    fn calibrator_state(&self) -> AlpacaResult<CalibratorState> { Ok(CalibratorState::Off) }
    fn cover_state(&self) -> AlpacaResult<CoverState> { Ok(CoverState::Closed) }
    fn calibrator_on(&self, _brightness: i32) -> AlpacaResult<()> { Ok(()) }
    fn calibrator_off(&self) -> AlpacaResult<()> { Ok(()) }
    fn open_cover(&self) -> AlpacaResult<()> { Ok(()) }
    fn close_cover(&self) -> AlpacaResult<()> { Ok(()) }
    fn halt_cover(&self) -> AlpacaResult<()> { Ok(()) }
    fn calibrator_changing(&self) -> AlpacaResult<bool> { Ok(false) }
    fn cover_moving(&self) -> AlpacaResult<bool> { Ok(false) }
}
