use std::sync::Mutex;
use std::time::Instant;

use crate::camera::{Camera, CameraState, GuideDirection, SensorType};
use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

/// Feature flags for optional Camera capabilities.
#[derive(Default)]
pub struct CameraFeatures {
    pub cooler: bool,
    pub pulse_guide: bool,
    pub fast_readout: bool,
    pub asymmetric_bin: bool,
    pub gain_mode: bool,
    pub offset_mode: bool,
    pub shutter: bool,
    pub sub_exposure: bool,
}

pub struct MockCamera {
    features: CameraFeatures,
    connected: Mutex<bool>,
    state: Mutex<CameraState>,
    image_ready: Mutex<bool>,
    exposure_start: Mutex<Option<Instant>>,
    exposure_duration_secs: Mutex<f64>,
    bin_x: Mutex<i32>,
    bin_y: Mutex<i32>,
    start_x: Mutex<i32>,
    start_y: Mutex<i32>,
    num_x: Mutex<i32>,
    num_y: Mutex<i32>,
    // Cooler state (when features.cooler = true)
    cooler_on: Mutex<bool>,
    target_temp: Mutex<f64>,
    // Gain/offset state
    gain: Mutex<i32>,
    offset: Mutex<i32>,
    // Fast readout
    fast_readout: Mutex<bool>,
    // Readout mode
    readout_mode: Mutex<i32>,
    // Sub exposure
    sub_exposure_duration: Mutex<f64>,
}

impl MockCamera {
    pub fn new() -> Self {
        Self::with_features(CameraFeatures::default())
    }

    pub fn with_features(features: CameraFeatures) -> Self {
        Self {
            features,
            connected: Mutex::new(false),
            state: Mutex::new(CameraState::Idle),
            image_ready: Mutex::new(false),
            exposure_start: Mutex::new(None),
            exposure_duration_secs: Mutex::new(0.0),
            bin_x: Mutex::new(1),
            bin_y: Mutex::new(1),
            start_x: Mutex::new(0),
            start_y: Mutex::new(0),
            num_x: Mutex::new(1024),
            num_y: Mutex::new(768),
            cooler_on: Mutex::new(false),
            target_temp: Mutex::new(-10.0),
            gain: Mutex::new(0),
            offset: Mutex::new(0),
            fast_readout: Mutex::new(false),
            readout_mode: Mutex::new(0),
            sub_exposure_duration: Mutex::new(0.0),
        }
    }

    /// Check if a running exposure has completed based on wall clock time.
    fn check_exposure_complete(&self) {
        let start = *self.exposure_start.lock().unwrap();
        if let Some(started_at) = start {
            let duration = *self.exposure_duration_secs.lock().unwrap();
            if started_at.elapsed().as_secs_f64() >= duration {
                *self.state.lock().unwrap() = CameraState::Idle;
                *self.image_ready.lock().unwrap() = true;
                *self.exposure_start.lock().unwrap() = None;
            }
        }
    }

    /// All features enabled — for full ConformU sweep.
    pub fn full_featured() -> Self {
        Self::with_features(CameraFeatures {
            cooler: true,
            pulse_guide: true,
            fast_readout: true,
            asymmetric_bin: true,
            gain_mode: true,
            offset_mode: true,
            shutter: true,
            sub_exposure: true,
        })
    }
}

impl Device for MockCamera {
    fn static_name(&self) -> &str { "Mock Camera" }
    fn unique_id(&self) -> &str { "mock-cam-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Camera }
    fn connected(&self) -> AlpacaResult<bool> { Ok(*self.connected.lock().unwrap()) }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> { *self.connected.lock().unwrap() = v; Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = true; Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = false; Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock Camera for ConformU testing".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock driver".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(3) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Camera".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl Camera for MockCamera {
    // --- Mandatory properties (always available) ---

    fn camera_xsize(&self) -> AlpacaResult<i32> { Ok(1024) }
    fn camera_ysize(&self) -> AlpacaResult<i32> { Ok(768) }
    fn max_adu(&self) -> AlpacaResult<i32> { Ok(65535) }
    fn sensor_name(&self) -> AlpacaResult<String> { Ok("Mock Sensor".into()) }
    fn sensor_type(&self) -> AlpacaResult<SensorType> { Ok(SensorType::Monochrome) }
    fn pixel_size_x(&self) -> AlpacaResult<f64> { Ok(3.75) }
    fn pixel_size_y(&self) -> AlpacaResult<f64> { Ok(3.75) }
    fn electrons_per_adu(&self) -> AlpacaResult<f64> { Ok(1.0) }
    fn full_well_capacity(&self) -> AlpacaResult<f64> { Ok(65535.0) }
    fn bayer_offset_x(&self) -> AlpacaResult<i32> {
        if self.sensor_type()? == SensorType::Monochrome {
            Err(AlpacaError::NotImplemented("BayerOffsetX not available for monochrome sensor".into()))
        } else { Ok(0) }
    }
    fn bayer_offset_y(&self) -> AlpacaResult<i32> {
        if self.sensor_type()? == SensorType::Monochrome {
            Err(AlpacaError::NotImplemented("BayerOffsetY not available for monochrome sensor".into()))
        } else { Ok(0) }
    }
    fn exposure_min(&self) -> AlpacaResult<f64> { Ok(0.001) }
    fn exposure_max(&self) -> AlpacaResult<f64> { Ok(3600.0) }
    fn exposure_resolution(&self) -> AlpacaResult<f64> { Ok(0.001) }
    fn percent_completed(&self) -> AlpacaResult<i32> { Ok(0) }
    fn ccd_temperature(&self) -> AlpacaResult<f64> { Ok(-10.0) }
    fn heat_sink_temperature(&self) -> AlpacaResult<f64> { Ok(25.0) }

    // --- Binning (always available, mandatory) ---

    fn bin_x(&self) -> AlpacaResult<i32> { Ok(*self.bin_x.lock().unwrap()) }
    fn set_bin_x(&self, v: i32) -> AlpacaResult<()> {
        if v < 1 || v > 4 { return Err(AlpacaError::InvalidValue(format!("BinX must be 1-4, got {v}"))); }
        *self.bin_x.lock().unwrap() = v;
        Ok(())
    }
    fn bin_y(&self) -> AlpacaResult<i32> { Ok(*self.bin_y.lock().unwrap()) }
    fn set_bin_y(&self, v: i32) -> AlpacaResult<()> {
        if v < 1 || v > 4 { return Err(AlpacaError::InvalidValue(format!("BinY must be 1-4, got {v}"))); }
        *self.bin_y.lock().unwrap() = v;
        Ok(())
    }
    fn max_bin_x(&self) -> AlpacaResult<i32> { Ok(4) }
    fn max_bin_y(&self) -> AlpacaResult<i32> { Ok(4) }

    // --- Subframe (always available) ---

    fn start_x(&self) -> AlpacaResult<i32> { Ok(*self.start_x.lock().unwrap()) }
    fn set_start_x(&self, v: i32) -> AlpacaResult<()> {
        let max = 1024 / *self.bin_x.lock().unwrap();
        if v < 0 || v >= max { return Err(AlpacaError::InvalidValue(format!("StartX {v} out of range 0-{}", max - 1))); }
        *self.start_x.lock().unwrap() = v; Ok(())
    }
    fn start_y(&self) -> AlpacaResult<i32> { Ok(*self.start_y.lock().unwrap()) }
    fn set_start_y(&self, v: i32) -> AlpacaResult<()> {
        let max = 768 / *self.bin_y.lock().unwrap();
        if v < 0 || v >= max { return Err(AlpacaError::InvalidValue(format!("StartY {v} out of range 0-{}", max - 1))); }
        *self.start_y.lock().unwrap() = v; Ok(())
    }
    fn num_x(&self) -> AlpacaResult<i32> { Ok(*self.num_x.lock().unwrap()) }
    fn set_num_x(&self, v: i32) -> AlpacaResult<()> {
        let max = 1024 / *self.bin_x.lock().unwrap() - *self.start_x.lock().unwrap();
        if v < 1 || v > max { return Err(AlpacaError::InvalidValue(format!("NumX {v} out of range 1-{max}"))); }
        *self.num_x.lock().unwrap() = v; Ok(())
    }
    fn num_y(&self) -> AlpacaResult<i32> { Ok(*self.num_y.lock().unwrap()) }
    fn set_num_y(&self, v: i32) -> AlpacaResult<()> {
        let max = 768 / *self.bin_y.lock().unwrap() - *self.start_y.lock().unwrap();
        if v < 1 || v > max { return Err(AlpacaError::InvalidValue(format!("NumY {v} out of range 1-{max}"))); }
        *self.num_y.lock().unwrap() = v; Ok(())
    }

    // --- Exposure (always available) ---

    fn image_ready(&self) -> AlpacaResult<bool> {
        self.check_exposure_complete();
        Ok(*self.image_ready.lock().unwrap())
    }
    fn camera_state(&self) -> AlpacaResult<CameraState> {
        self.check_exposure_complete();
        Ok(*self.state.lock().unwrap())
    }
    fn start_exposure(&self, duration: f64, _light: bool) -> AlpacaResult<()> {
        if duration < 0.0 {
            return Err(AlpacaError::InvalidValue(format!("Duration must be >= 0, got {duration}")));
        }
        *self.image_ready.lock().unwrap() = false;
        *self.exposure_duration_secs.lock().unwrap() = duration;
        *self.exposure_start.lock().unwrap() = Some(Instant::now());
        *self.state.lock().unwrap() = CameraState::Exposing;
        Ok(())
    }
    fn stop_exposure(&self) -> AlpacaResult<()> {
        *self.state.lock().unwrap() = CameraState::Idle;
        *self.image_ready.lock().unwrap() = true;
        *self.exposure_start.lock().unwrap() = None;
        Ok(())
    }
    fn abort_exposure(&self) -> AlpacaResult<()> {
        *self.state.lock().unwrap() = CameraState::Idle;
        *self.image_ready.lock().unwrap() = false;
        *self.exposure_start.lock().unwrap() = None;
        Ok(())
    }
    fn can_abort_exposure(&self) -> AlpacaResult<bool> { Ok(true) }
    fn can_stop_exposure(&self) -> AlpacaResult<bool> { Ok(true) }

    // --- Readout mode (always available) ---

    fn readout_mode(&self) -> AlpacaResult<i32> { Ok(*self.readout_mode.lock().unwrap()) }
    fn set_readout_mode(&self, v: i32) -> AlpacaResult<()> { *self.readout_mode.lock().unwrap() = v; Ok(()) }
    fn readout_modes(&self) -> AlpacaResult<Vec<String>> { Ok(vec!["Default".into()]) }

    // --- Feature-gated: asymmetric bin ---

    fn can_asymmetric_bin(&self) -> AlpacaResult<bool> { Ok(self.features.asymmetric_bin) }

    // --- Feature-gated: shutter ---

    fn has_shutter(&self) -> AlpacaResult<bool> { Ok(self.features.shutter) }

    // --- Feature-gated: cooler ---

    fn can_set_ccd_temperature(&self) -> AlpacaResult<bool> { Ok(self.features.cooler) }
    fn can_get_cooler_power(&self) -> AlpacaResult<bool> { Ok(self.features.cooler) }
    fn cooler_on(&self) -> AlpacaResult<bool> {
        if !self.features.cooler { return Err(AlpacaError::NotImplemented("cooler_on".into())); }
        Ok(*self.cooler_on.lock().unwrap())
    }
    fn set_cooler_on(&self, on: bool) -> AlpacaResult<()> {
        if !self.features.cooler { return Err(AlpacaError::NotImplemented("set_cooler_on".into())); }
        *self.cooler_on.lock().unwrap() = on;
        Ok(())
    }
    fn cooler_power(&self) -> AlpacaResult<f64> {
        if !self.features.cooler { return Err(AlpacaError::NotImplemented("cooler_power".into())); }
        Ok(if *self.cooler_on.lock().unwrap() { 50.0 } else { 0.0 })
    }
    fn set_ccd_temperature(&self) -> AlpacaResult<f64> {
        if !self.features.cooler { return Err(AlpacaError::NotImplemented("set_ccd_temperature".into())); }
        Ok(*self.target_temp.lock().unwrap())
    }
    fn set_set_ccd_temperature(&self, temp: f64) -> AlpacaResult<()> {
        if !self.features.cooler { return Err(AlpacaError::NotImplemented("set_set_ccd_temperature".into())); }
        *self.target_temp.lock().unwrap() = temp;
        Ok(())
    }

    // --- Feature-gated: pulse guide ---

    fn can_pulse_guide(&self) -> AlpacaResult<bool> { Ok(self.features.pulse_guide) }
    fn is_pulse_guiding(&self) -> AlpacaResult<bool> {
        if !self.features.pulse_guide { return Err(AlpacaError::NotImplemented("is_pulse_guiding".into())); }
        Ok(false)
    }
    fn pulse_guide(&self, _direction: GuideDirection, _duration: i32) -> AlpacaResult<()> {
        if !self.features.pulse_guide { return Err(AlpacaError::NotImplemented("pulse_guide".into())); }
        Ok(())
    }

    // --- Feature-gated: fast readout ---

    fn can_fast_readout(&self) -> AlpacaResult<bool> { Ok(self.features.fast_readout) }
    fn fast_readout(&self) -> AlpacaResult<bool> {
        if !self.features.fast_readout { return Err(AlpacaError::NotImplemented("fast_readout".into())); }
        Ok(*self.fast_readout.lock().unwrap())
    }
    fn set_fast_readout(&self, fast: bool) -> AlpacaResult<()> {
        if !self.features.fast_readout { return Err(AlpacaError::NotImplemented("set_fast_readout".into())); }
        *self.fast_readout.lock().unwrap() = fast;
        Ok(())
    }

    // --- Feature-gated: gain ---

    fn gain(&self) -> AlpacaResult<i32> {
        if !self.features.gain_mode { return Err(AlpacaError::NotImplemented("gain".into())); }
        Ok(*self.gain.lock().unwrap())
    }
    fn set_gain(&self, v: i32) -> AlpacaResult<()> {
        if !self.features.gain_mode { return Err(AlpacaError::NotImplemented("set_gain".into())); }
        *self.gain.lock().unwrap() = v;
        Ok(())
    }
    fn gain_min(&self) -> AlpacaResult<i32> {
        if !self.features.gain_mode { return Err(AlpacaError::NotImplemented("gain_min".into())); }
        Ok(0)
    }
    fn gain_max(&self) -> AlpacaResult<i32> {
        if !self.features.gain_mode { return Err(AlpacaError::NotImplemented("gain_max".into())); }
        Ok(100)
    }
    fn gains(&self) -> AlpacaResult<Vec<String>> {
        if !self.features.gain_mode { return Err(AlpacaError::NotImplemented("gains".into())); }
        Err(AlpacaError::NotImplemented("gains list not supported in numeric gain mode".into()))
    }

    // --- Feature-gated: offset ---

    fn offset(&self) -> AlpacaResult<i32> {
        if !self.features.offset_mode { return Err(AlpacaError::NotImplemented("offset".into())); }
        Ok(*self.offset.lock().unwrap())
    }
    fn set_offset(&self, v: i32) -> AlpacaResult<()> {
        if !self.features.offset_mode { return Err(AlpacaError::NotImplemented("set_offset".into())); }
        *self.offset.lock().unwrap() = v;
        Ok(())
    }
    fn offset_min(&self) -> AlpacaResult<i32> {
        if !self.features.offset_mode { return Err(AlpacaError::NotImplemented("offset_min".into())); }
        Ok(0)
    }
    fn offset_max(&self) -> AlpacaResult<i32> {
        if !self.features.offset_mode { return Err(AlpacaError::NotImplemented("offset_max".into())); }
        Ok(100)
    }

    // --- Feature-gated: sub exposure ---

    fn sub_exposure_duration(&self) -> AlpacaResult<f64> {
        if !self.features.sub_exposure { return Err(AlpacaError::NotImplemented("sub_exposure_duration".into())); }
        Ok(*self.sub_exposure_duration.lock().unwrap())
    }
    fn set_sub_exposure_duration(&self, v: f64) -> AlpacaResult<()> {
        if !self.features.sub_exposure { return Err(AlpacaError::NotImplemented("set_sub_exposure_duration".into())); }
        *self.sub_exposure_duration.lock().unwrap() = v;
        Ok(())
    }
}
