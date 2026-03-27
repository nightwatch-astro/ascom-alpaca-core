use std::sync::Mutex;

use crate::camera::{Camera, CameraState, SensorType};
use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

pub struct MockCamera {
    state: Mutex<CameraState>,
    bin_x: Mutex<i32>,
    bin_y: Mutex<i32>,
    start_x: Mutex<i32>,
    start_y: Mutex<i32>,
    num_x: Mutex<i32>,
    num_y: Mutex<i32>,
}

impl MockCamera {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(CameraState::Idle),
            bin_x: Mutex::new(1),
            bin_y: Mutex::new(1),
            start_x: Mutex::new(0),
            start_y: Mutex::new(0),
            num_x: Mutex::new(1024),
            num_y: Mutex::new(768),
        }
    }
}

impl Device for MockCamera {
    fn static_name(&self) -> &str { "Mock Camera" }
    fn unique_id(&self) -> &str { "mock-cam-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Camera }
    fn connected(&self) -> AlpacaResult<bool> { Ok(true) }
    fn set_connected(&self, _: bool) -> AlpacaResult<()> { Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock Camera for ConformU testing".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock driver".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(3) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Camera".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl Camera for MockCamera {
    fn camera_state(&self) -> AlpacaResult<CameraState> { Ok(*self.state.lock().unwrap()) }
    fn camera_xsize(&self) -> AlpacaResult<i32> { Ok(1024) }
    fn camera_ysize(&self) -> AlpacaResult<i32> { Ok(768) }
    fn max_adu(&self) -> AlpacaResult<i32> { Ok(65535) }
    fn sensor_name(&self) -> AlpacaResult<String> { Ok("Mock Sensor".into()) }
    fn sensor_type(&self) -> AlpacaResult<SensorType> { Ok(SensorType::Monochrome) }
    fn pixel_size_x(&self) -> AlpacaResult<f64> { Ok(3.75) }
    fn pixel_size_y(&self) -> AlpacaResult<f64> { Ok(3.75) }
    fn bin_x(&self) -> AlpacaResult<i32> { Ok(*self.bin_x.lock().unwrap()) }
    fn set_bin_x(&self, v: i32) -> AlpacaResult<()> { *self.bin_x.lock().unwrap() = v; Ok(()) }
    fn bin_y(&self) -> AlpacaResult<i32> { Ok(*self.bin_y.lock().unwrap()) }
    fn set_bin_y(&self, v: i32) -> AlpacaResult<()> { *self.bin_y.lock().unwrap() = v; Ok(()) }
    fn max_bin_x(&self) -> AlpacaResult<i32> { Ok(4) }
    fn max_bin_y(&self) -> AlpacaResult<i32> { Ok(4) }
    fn can_asymmetric_bin(&self) -> AlpacaResult<bool> { Ok(false) }
    fn start_x(&self) -> AlpacaResult<i32> { Ok(*self.start_x.lock().unwrap()) }
    fn set_start_x(&self, v: i32) -> AlpacaResult<()> { *self.start_x.lock().unwrap() = v; Ok(()) }
    fn start_y(&self) -> AlpacaResult<i32> { Ok(*self.start_y.lock().unwrap()) }
    fn set_start_y(&self, v: i32) -> AlpacaResult<()> { *self.start_y.lock().unwrap() = v; Ok(()) }
    fn num_x(&self) -> AlpacaResult<i32> { Ok(*self.num_x.lock().unwrap()) }
    fn set_num_x(&self, v: i32) -> AlpacaResult<()> { *self.num_x.lock().unwrap() = v; Ok(()) }
    fn num_y(&self) -> AlpacaResult<i32> { Ok(*self.num_y.lock().unwrap()) }
    fn set_num_y(&self, v: i32) -> AlpacaResult<()> { *self.num_y.lock().unwrap() = v; Ok(()) }
    fn exposure_min(&self) -> AlpacaResult<f64> { Ok(0.001) }
    fn exposure_max(&self) -> AlpacaResult<f64> { Ok(3600.0) }
    fn exposure_resolution(&self) -> AlpacaResult<f64> { Ok(0.001) }
    fn can_abort_exposure(&self) -> AlpacaResult<bool> { Ok(true) }
    fn can_stop_exposure(&self) -> AlpacaResult<bool> { Ok(true) }
    fn has_shutter(&self) -> AlpacaResult<bool> { Ok(false) }
    fn image_ready(&self) -> AlpacaResult<bool> { Ok(false) }
    fn start_exposure(&self, _duration: f64, _light: bool) -> AlpacaResult<()> {
        *self.state.lock().unwrap() = CameraState::Exposing;
        Ok(())
    }
    fn stop_exposure(&self) -> AlpacaResult<()> {
        *self.state.lock().unwrap() = CameraState::Idle;
        Ok(())
    }
    fn abort_exposure(&self) -> AlpacaResult<()> {
        *self.state.lock().unwrap() = CameraState::Idle;
        Ok(())
    }
    fn electrons_per_adu(&self) -> AlpacaResult<f64> { Ok(1.0) }
    fn full_well_capacity(&self) -> AlpacaResult<f64> { Ok(65535.0) }
    fn bayer_offset_x(&self) -> AlpacaResult<i32> { Ok(0) }
    fn bayer_offset_y(&self) -> AlpacaResult<i32> { Ok(0) }
    fn readout_mode(&self) -> AlpacaResult<i32> { Ok(0) }
    fn set_readout_mode(&self, _: i32) -> AlpacaResult<()> { Ok(()) }
    fn readout_modes(&self) -> AlpacaResult<Vec<String>> { Ok(vec!["Default".into()]) }
    fn percent_completed(&self) -> AlpacaResult<i32> { Ok(0) }
    fn gain(&self) -> AlpacaResult<i32> { Err(AlpacaError::NotImplemented("gain".into())) }
    fn offset(&self) -> AlpacaResult<i32> { Err(AlpacaError::NotImplemented("offset".into())) }
    fn ccd_temperature(&self) -> AlpacaResult<f64> { Ok(-10.0) }
    fn heat_sink_temperature(&self) -> AlpacaResult<f64> { Ok(25.0) }
    fn cooler_on(&self) -> AlpacaResult<bool> { Ok(false) }
    fn cooler_power(&self) -> AlpacaResult<f64> { Ok(0.0) }
    fn set_ccd_temperature(&self) -> AlpacaResult<f64> { Ok(-10.0) }
    fn can_pulse_guide(&self) -> AlpacaResult<bool> { Ok(false) }
    fn is_pulse_guiding(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_fast_readout(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_set_ccd_temperature(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_get_cooler_power(&self) -> AlpacaResult<bool> { Ok(true) }
}
