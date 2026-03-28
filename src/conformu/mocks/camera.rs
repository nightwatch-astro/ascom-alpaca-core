use std::sync::Mutex;
use std::time::Instant;

use crate::camera::{Camera, CameraState, GuideDirection, ImageData, SensorType};
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

/// Gain/offset mode selection — ASCOM cameras use exactly one of these.
/// Numeric: `gain/gain_min/gain_max` work, `gains()` → NotImplemented.
/// Named: `gains()` returns a list, `gain_min/gain_max` → NotImplemented,
///   `gain/set_gain` use indices into the names list.
#[derive(Clone)]
pub enum GainOffsetMode {
    /// Not supported — all methods return NotImplemented.
    None,
    /// Numeric range: gain/offset is a value in min..=max.
    Numeric { min: i32, max: i32 },
    /// Named list: gain/offset is an index into the names list.
    Named(Vec<String>),
}

/// Feature flags for optional Camera capabilities.
pub struct CameraFeatures {
    pub cooler: bool,
    pub pulse_guide: bool,
    pub fast_readout: bool,
    pub asymmetric_bin: bool,
    pub gain_mode: GainOffsetMode,
    pub offset_mode: GainOffsetMode,
    pub shutter: bool,
    pub sub_exposure: bool,
    /// Sensor type: Monochrome (default), Color, RGGB, CMYG, CMYG2, or LRGB.
    pub sensor_type: SensorType,
}

impl Default for CameraFeatures {
    fn default() -> Self {
        Self {
            cooler: false,
            pulse_guide: false,
            fast_readout: false,
            asymmetric_bin: false,
            gain_mode: GainOffsetMode::None,
            offset_mode: GainOffsetMode::None,
            shutter: false,
            sub_exposure: false,
            sensor_type: SensorType::Monochrome,
        }
    }
}

pub struct MockCamera {
    features: CameraFeatures,
    unique_id: String,
    camera_name: String,
    connected: Mutex<bool>,
    state: Mutex<CameraState>,
    image_ready: Mutex<bool>,
    exposure_start: Mutex<Option<Instant>>,
    exposure_duration_secs: Mutex<f64>,
    last_duration: Mutex<Option<f64>>,
    last_start_time: Mutex<Option<String>>,
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
    // Pulse guide timing
    pulse_guide_start: Mutex<Option<Instant>>,
    pulse_guide_duration_ms: Mutex<i32>,
}

impl Default for MockCamera {
    fn default() -> Self {
        Self::new()
    }
}

impl MockCamera {
    pub fn new() -> Self {
        Self::with_features(CameraFeatures::default())
    }

    pub fn with_features(features: CameraFeatures) -> Self {
        Self::with_features_and_id(features, "mock-cam-001", "Mock Camera")
    }

    pub fn with_features_and_id(features: CameraFeatures, id: &str, name: &str) -> Self {
        Self {
            features,
            unique_id: id.to_string(),
            camera_name: name.to_string(),
            connected: Mutex::new(false),
            state: Mutex::new(CameraState::Idle),
            image_ready: Mutex::new(false),
            exposure_start: Mutex::new(None),
            exposure_duration_secs: Mutex::new(0.0),
            last_duration: Mutex::new(None),
            last_start_time: Mutex::new(None),
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
            sub_exposure_duration: Mutex::new(1.0),
            pulse_guide_start: Mutex::new(None),
            pulse_guide_duration_ms: Mutex::new(0),
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
                *self.last_duration.lock().unwrap() = Some(duration);
                *self.exposure_start.lock().unwrap() = None;
            }
        }
    }

    /// All features enabled with numeric gain/offset — for full ConformU sweep.
    pub fn full_featured() -> Self {
        Self::with_features(CameraFeatures {
            cooler: true,
            pulse_guide: true,
            fast_readout: true,
            asymmetric_bin: true,
            gain_mode: GainOffsetMode::Numeric { min: 0, max: 100 },
            offset_mode: GainOffsetMode::Numeric { min: 0, max: 100 },
            shutter: true,
            sub_exposure: true,
            sensor_type: SensorType::Monochrome,
        })
    }
}

/// Simple epoch-days-to-date conversion (no chrono dependency).
fn epoch_days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

impl_mock_device!(MockCamera,
    name_field: camera_name,
    unique_id_field: unique_id,
    device_type: DeviceType::Camera,
    interface_version: 4,
    device_state: |self_: &MockCamera| {
        use crate::device::common::DeviceStateBuilder;
        self_.check_exposure_complete();
        let state = *self_.state.lock().unwrap();
        let mut b = DeviceStateBuilder::new()
            .add("CameraState", state as i32)
            .add("CCDTemperature", -10.0)
            .add("ImageReady", *self_.image_ready.lock().unwrap())
            .add("HeatSinkTemperature", 25.0)
            .add("IsPulseGuiding", false)
            .add("PercentCompleted", 0);
        if self_.features.cooler {
            b = b.add(
                "CoolerPower",
                if *self_.cooler_on.lock().unwrap() { 50.0 } else { 0.0 },
            );
        }
        Ok(b.build())
    }
);

impl Camera for MockCamera {
    // --- Mandatory properties (always available) ---

    fn camera_xsize(&self) -> AlpacaResult<i32> {
        Ok(1024)
    }
    fn camera_ysize(&self) -> AlpacaResult<i32> {
        Ok(768)
    }
    fn max_adu(&self) -> AlpacaResult<i32> {
        Ok(65535)
    }
    fn sensor_name(&self) -> AlpacaResult<String> {
        Ok("Mock Sensor".into())
    }
    fn sensor_type(&self) -> AlpacaResult<SensorType> {
        Ok(self.features.sensor_type)
    }
    fn pixel_size_x(&self) -> AlpacaResult<f64> {
        Ok(3.75)
    }
    fn pixel_size_y(&self) -> AlpacaResult<f64> {
        Ok(3.75)
    }
    fn electrons_per_adu(&self) -> AlpacaResult<f64> {
        Ok(1.0)
    }
    fn full_well_capacity(&self) -> AlpacaResult<f64> {
        Ok(65535.0)
    }
    fn bayer_offset_x(&self) -> AlpacaResult<i32> {
        if self.features.sensor_type == SensorType::Monochrome {
            Err(AlpacaError::NotImplemented(
                "BayerOffsetX not available for monochrome sensor".into(),
            ))
        } else {
            Ok(0)
        }
    }
    fn bayer_offset_y(&self) -> AlpacaResult<i32> {
        if self.features.sensor_type == SensorType::Monochrome {
            Err(AlpacaError::NotImplemented(
                "BayerOffsetY not available for monochrome sensor".into(),
            ))
        } else {
            Ok(0)
        }
    }
    fn image_array(&self) -> AlpacaResult<ImageData> {
        self.check_exposure_complete();
        if !*self.image_ready.lock().unwrap() {
            return Err(AlpacaError::InvalidOperationException(
                "No image available".into(),
            ));
        }
        let nx = *self.num_x.lock().unwrap() as usize;
        let ny = *self.num_y.lock().unwrap() as usize;
        // ASCOM convention: ImageArray[x][y] — first dimension is X (columns)
        let data: Vec<Vec<i32>> = (0..nx)
            .map(|x| (0..ny).map(|y| ((x + y) % 65536) as i32).collect())
            .collect();
        Ok(ImageData::I32_2D(data))
    }

    fn last_exposure_duration(&self) -> AlpacaResult<f64> {
        self.last_duration.lock().unwrap().ok_or_else(|| {
            AlpacaError::InvalidOperationException("No exposure has been taken".into())
        })
    }

    fn last_exposure_start_time(&self) -> AlpacaResult<String> {
        self.last_start_time.lock().unwrap().clone().ok_or_else(|| {
            AlpacaError::InvalidOperationException("No exposure has been taken".into())
        })
    }

    fn exposure_min(&self) -> AlpacaResult<f64> {
        Ok(0.001)
    }
    fn exposure_max(&self) -> AlpacaResult<f64> {
        Ok(3600.0)
    }
    fn exposure_resolution(&self) -> AlpacaResult<f64> {
        Ok(0.001)
    }
    fn percent_completed(&self) -> AlpacaResult<i32> {
        Ok(0)
    }
    fn ccd_temperature(&self) -> AlpacaResult<f64> {
        Ok(-10.0)
    }
    fn heat_sink_temperature(&self) -> AlpacaResult<f64> {
        Ok(25.0)
    }

    // --- Binning (always available, mandatory) ---

    fn bin_x(&self) -> AlpacaResult<i32> {
        Ok(*self.bin_x.lock().unwrap())
    }
    fn set_bin_x(&self, v: i32) -> AlpacaResult<()> {
        if !(1..=4).contains(&v) {
            return Err(AlpacaError::InvalidValue(format!(
                "BinX must be 1-4, got {v}"
            )));
        }
        *self.bin_x.lock().unwrap() = v;
        // Reset subframe to full frame at new binning
        *self.start_x.lock().unwrap() = 0;
        *self.num_x.lock().unwrap() = 1024 / v;
        Ok(())
    }
    fn bin_y(&self) -> AlpacaResult<i32> {
        Ok(*self.bin_y.lock().unwrap())
    }
    fn set_bin_y(&self, v: i32) -> AlpacaResult<()> {
        if !(1..=4).contains(&v) {
            return Err(AlpacaError::InvalidValue(format!(
                "BinY must be 1-4, got {v}"
            )));
        }
        *self.bin_y.lock().unwrap() = v;
        // Reset subframe to full frame at new binning
        *self.start_y.lock().unwrap() = 0;
        *self.num_y.lock().unwrap() = 768 / v;
        Ok(())
    }
    fn max_bin_x(&self) -> AlpacaResult<i32> {
        Ok(4)
    }
    fn max_bin_y(&self) -> AlpacaResult<i32> {
        Ok(4)
    }

    // --- Subframe (always available) ---

    fn start_x(&self) -> AlpacaResult<i32> {
        Ok(*self.start_x.lock().unwrap())
    }
    fn set_start_x(&self, v: i32) -> AlpacaResult<()> {
        if v < 0 {
            return Err(AlpacaError::InvalidValue(format!(
                "StartX must be >= 0, got {v}"
            )));
        }
        *self.start_x.lock().unwrap() = v;
        Ok(())
    }
    fn start_y(&self) -> AlpacaResult<i32> {
        Ok(*self.start_y.lock().unwrap())
    }
    fn set_start_y(&self, v: i32) -> AlpacaResult<()> {
        if v < 0 {
            return Err(AlpacaError::InvalidValue(format!(
                "StartY must be >= 0, got {v}"
            )));
        }
        *self.start_y.lock().unwrap() = v;
        Ok(())
    }
    fn num_x(&self) -> AlpacaResult<i32> {
        Ok(*self.num_x.lock().unwrap())
    }
    fn set_num_x(&self, v: i32) -> AlpacaResult<()> {
        if v < 1 {
            return Err(AlpacaError::InvalidValue(format!(
                "NumX must be >= 1, got {v}"
            )));
        }
        *self.num_x.lock().unwrap() = v;
        Ok(())
    }
    fn num_y(&self) -> AlpacaResult<i32> {
        Ok(*self.num_y.lock().unwrap())
    }
    fn set_num_y(&self, v: i32) -> AlpacaResult<()> {
        if v < 1 {
            return Err(AlpacaError::InvalidValue(format!(
                "NumY must be >= 1, got {v}"
            )));
        }
        *self.num_y.lock().unwrap() = v;
        Ok(())
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
            return Err(AlpacaError::InvalidValue(format!(
                "Duration must be >= 0, got {duration}"
            )));
        }
        // Validate subframe fits within sensor at current binning
        let bx = *self.bin_x.lock().unwrap();
        let by = *self.bin_y.lock().unwrap();
        let sx = *self.start_x.lock().unwrap();
        let sy = *self.start_y.lock().unwrap();
        let nx = *self.num_x.lock().unwrap();
        let ny = *self.num_y.lock().unwrap();
        let max_x = 1024 / bx;
        let max_y = 768 / by;
        if sx < 0 || sx >= max_x {
            return Err(AlpacaError::InvalidValue(format!(
                "StartX {sx} out of range for current binning"
            )));
        }
        if sy < 0 || sy >= max_y {
            return Err(AlpacaError::InvalidValue(format!(
                "StartY {sy} out of range for current binning"
            )));
        }
        if nx < 1 || sx + nx > max_x {
            return Err(AlpacaError::InvalidValue(format!(
                "NumX {nx} out of range for current binning and StartX"
            )));
        }
        if ny < 1 || sy + ny > max_y {
            return Err(AlpacaError::InvalidValue(format!(
                "NumY {ny} out of range for current binning and StartY"
            )));
        }
        *self.image_ready.lock().unwrap() = false;
        *self.exposure_duration_secs.lock().unwrap() = duration;
        *self.exposure_start.lock().unwrap() = Some(Instant::now());
        *self.state.lock().unwrap() = CameraState::Exposing;
        // Record ISO 8601 start time (FITS-style, UTC approximation)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        // Simple UTC timestamp without chrono dependency
        let secs_per_day = 86400u64;
        let days_since_epoch = now / secs_per_day;
        let time_of_day = now % secs_per_day;
        let hours = time_of_day / 3600;
        let minutes = (time_of_day % 3600) / 60;
        let seconds = time_of_day % 60;
        // Approximate date calculation (good enough for mock)
        let (year, month, day) = epoch_days_to_ymd(days_since_epoch);
        *self.last_start_time.lock().unwrap() = Some(format!(
            "{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}"
        ));
        *self.last_duration.lock().unwrap() = Some(duration);
        Ok(())
    }
    fn stop_exposure(&self) -> AlpacaResult<()> {
        let current_state = *self.state.lock().unwrap();
        match current_state {
            CameraState::Exposing => {
                *self.state.lock().unwrap() = CameraState::Idle;
                *self.image_ready.lock().unwrap() = true;
                *self.exposure_start.lock().unwrap() = None;
            }
            CameraState::Idle => {
                // No-op when idle — ConformU expects this to succeed silently
            }
            _ => {
                return Err(AlpacaError::InvalidOperationException(
                    "Cannot stop exposure in current state".into(),
                ));
            }
        }
        Ok(())
    }
    fn abort_exposure(&self) -> AlpacaResult<()> {
        *self.state.lock().unwrap() = CameraState::Idle;
        *self.image_ready.lock().unwrap() = false;
        *self.exposure_start.lock().unwrap() = None;
        Ok(())
    }
    fn can_abort_exposure(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_stop_exposure(&self) -> AlpacaResult<bool> {
        Ok(true)
    }

    // --- Readout mode (always available) ---

    fn readout_mode(&self) -> AlpacaResult<i32> {
        Ok(*self.readout_mode.lock().unwrap())
    }
    fn set_readout_mode(&self, v: i32) -> AlpacaResult<()> {
        *self.readout_mode.lock().unwrap() = v;
        Ok(())
    }
    fn readout_modes(&self) -> AlpacaResult<Vec<String>> {
        Ok(vec!["Default".into()])
    }

    // --- Feature-gated: asymmetric bin ---

    fn can_asymmetric_bin(&self) -> AlpacaResult<bool> {
        Ok(self.features.asymmetric_bin)
    }

    // --- Feature-gated: shutter ---

    fn has_shutter(&self) -> AlpacaResult<bool> {
        Ok(self.features.shutter)
    }

    // --- Feature-gated: cooler ---

    fn can_set_ccd_temperature(&self) -> AlpacaResult<bool> {
        Ok(self.features.cooler)
    }
    fn can_get_cooler_power(&self) -> AlpacaResult<bool> {
        Ok(self.features.cooler)
    }
    fn cooler_on(&self) -> AlpacaResult<bool> {
        if !self.features.cooler {
            return Err(AlpacaError::NotImplemented("cooler_on".into()));
        }
        Ok(*self.cooler_on.lock().unwrap())
    }
    fn set_cooler_on(&self, on: bool) -> AlpacaResult<()> {
        if !self.features.cooler {
            return Err(AlpacaError::NotImplemented("set_cooler_on".into()));
        }
        *self.cooler_on.lock().unwrap() = on;
        Ok(())
    }
    fn cooler_power(&self) -> AlpacaResult<f64> {
        if !self.features.cooler {
            return Err(AlpacaError::NotImplemented("cooler_power".into()));
        }
        Ok(if *self.cooler_on.lock().unwrap() {
            50.0
        } else {
            0.0
        })
    }
    fn set_ccd_temperature(&self) -> AlpacaResult<f64> {
        if !self.features.cooler {
            return Err(AlpacaError::NotImplemented("set_ccd_temperature".into()));
        }
        Ok(*self.target_temp.lock().unwrap())
    }
    fn set_set_ccd_temperature(&self, temp: f64) -> AlpacaResult<()> {
        if !self.features.cooler {
            return Err(AlpacaError::NotImplemented(
                "set_set_ccd_temperature".into(),
            ));
        }
        if temp < -273.15 {
            return Err(AlpacaError::InvalidValue(format!(
                "Temperature {temp}°C is below absolute zero"
            )));
        }
        if temp > 50.0 {
            return Err(AlpacaError::InvalidValue(format!(
                "Temperature {temp}°C is above maximum"
            )));
        }
        *self.target_temp.lock().unwrap() = temp;
        Ok(())
    }

    // --- Feature-gated: pulse guide ---

    fn can_pulse_guide(&self) -> AlpacaResult<bool> {
        Ok(self.features.pulse_guide)
    }
    fn is_pulse_guiding(&self) -> AlpacaResult<bool> {
        if !self.features.pulse_guide {
            return Err(AlpacaError::NotImplemented("is_pulse_guiding".into()));
        }
        let start = *self.pulse_guide_start.lock().unwrap();
        if let Some(started_at) = start {
            let dur = *self.pulse_guide_duration_ms.lock().unwrap();
            if started_at.elapsed().as_millis() < dur as u128 {
                return Ok(true);
            }
            *self.pulse_guide_start.lock().unwrap() = None;
        }
        Ok(false)
    }
    fn pulse_guide(&self, _direction: GuideDirection, duration: i32) -> AlpacaResult<()> {
        if !self.features.pulse_guide {
            return Err(AlpacaError::NotImplemented("pulse_guide".into()));
        }
        *self.pulse_guide_duration_ms.lock().unwrap() = duration;
        *self.pulse_guide_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    // --- Feature-gated: fast readout ---

    fn can_fast_readout(&self) -> AlpacaResult<bool> {
        Ok(self.features.fast_readout)
    }
    fn fast_readout(&self) -> AlpacaResult<bool> {
        if !self.features.fast_readout {
            return Err(AlpacaError::NotImplemented("fast_readout".into()));
        }
        Ok(*self.fast_readout.lock().unwrap())
    }
    fn set_fast_readout(&self, fast: bool) -> AlpacaResult<()> {
        if !self.features.fast_readout {
            return Err(AlpacaError::NotImplemented("set_fast_readout".into()));
        }
        *self.fast_readout.lock().unwrap() = fast;
        Ok(())
    }

    // --- Feature-gated: gain ---

    fn gain(&self) -> AlpacaResult<i32> {
        match &self.features.gain_mode {
            GainOffsetMode::None => Err(AlpacaError::NotImplemented("gain".into())),
            GainOffsetMode::Numeric { .. } | GainOffsetMode::Named(_) => {
                Ok(*self.gain.lock().unwrap())
            }
        }
    }
    fn set_gain(&self, v: i32) -> AlpacaResult<()> {
        match &self.features.gain_mode {
            GainOffsetMode::None => Err(AlpacaError::NotImplemented("set_gain".into())),
            GainOffsetMode::Numeric { min, max } => {
                if v < *min || v > *max {
                    return Err(AlpacaError::InvalidValue(format!(
                        "Gain {v} out of range {min}-{max}"
                    )));
                }
                *self.gain.lock().unwrap() = v;
                Ok(())
            }
            GainOffsetMode::Named(names) => {
                if v < 0 || v >= names.len() as i32 {
                    return Err(AlpacaError::InvalidValue(format!(
                        "Gain index {v} out of range 0-{}",
                        names.len() - 1
                    )));
                }
                *self.gain.lock().unwrap() = v;
                Ok(())
            }
        }
    }
    fn gain_min(&self) -> AlpacaResult<i32> {
        match &self.features.gain_mode {
            GainOffsetMode::Numeric { min, .. } => Ok(*min),
            _ => Err(AlpacaError::NotImplemented("gain_min".into())),
        }
    }
    fn gain_max(&self) -> AlpacaResult<i32> {
        match &self.features.gain_mode {
            GainOffsetMode::Numeric { max, .. } => Ok(*max),
            _ => Err(AlpacaError::NotImplemented("gain_max".into())),
        }
    }
    fn gains(&self) -> AlpacaResult<Vec<String>> {
        match &self.features.gain_mode {
            GainOffsetMode::Named(names) => Ok(names.clone()),
            _ => Err(AlpacaError::NotImplemented("gains".into())),
        }
    }

    // --- Feature-gated: offset ---

    fn offset(&self) -> AlpacaResult<i32> {
        match &self.features.offset_mode {
            GainOffsetMode::None => Err(AlpacaError::NotImplemented("offset".into())),
            GainOffsetMode::Numeric { .. } | GainOffsetMode::Named(_) => {
                Ok(*self.offset.lock().unwrap())
            }
        }
    }
    fn set_offset(&self, v: i32) -> AlpacaResult<()> {
        match &self.features.offset_mode {
            GainOffsetMode::None => Err(AlpacaError::NotImplemented("set_offset".into())),
            GainOffsetMode::Numeric { min, max } => {
                if v < *min || v > *max {
                    return Err(AlpacaError::InvalidValue(format!(
                        "Offset {v} out of range {min}-{max}"
                    )));
                }
                *self.offset.lock().unwrap() = v;
                Ok(())
            }
            GainOffsetMode::Named(names) => {
                if v < 0 || v >= names.len() as i32 {
                    return Err(AlpacaError::InvalidValue(format!(
                        "Offset index {v} out of range 0-{}",
                        names.len() - 1
                    )));
                }
                *self.offset.lock().unwrap() = v;
                Ok(())
            }
        }
    }
    fn offset_min(&self) -> AlpacaResult<i32> {
        match &self.features.offset_mode {
            GainOffsetMode::Numeric { min, .. } => Ok(*min),
            _ => Err(AlpacaError::NotImplemented("offset_min".into())),
        }
    }
    fn offset_max(&self) -> AlpacaResult<i32> {
        match &self.features.offset_mode {
            GainOffsetMode::Numeric { max, .. } => Ok(*max),
            _ => Err(AlpacaError::NotImplemented("offset_max".into())),
        }
    }
    fn offsets(&self) -> AlpacaResult<Vec<String>> {
        match &self.features.offset_mode {
            GainOffsetMode::Named(names) => Ok(names.clone()),
            _ => Err(AlpacaError::NotImplemented("offsets".into())),
        }
    }

    // --- Feature-gated: sub exposure ---

    fn sub_exposure_duration(&self) -> AlpacaResult<f64> {
        if !self.features.sub_exposure {
            return Err(AlpacaError::NotImplemented("sub_exposure_duration".into()));
        }
        Ok(*self.sub_exposure_duration.lock().unwrap())
    }
    fn set_sub_exposure_duration(&self, v: f64) -> AlpacaResult<()> {
        if !self.features.sub_exposure {
            return Err(AlpacaError::NotImplemented(
                "set_sub_exposure_duration".into(),
            ));
        }
        if v < 0.0 {
            return Err(AlpacaError::InvalidValue(format!(
                "SubExposureDuration must be >= 0, got {v}"
            )));
        }
        *self.sub_exposure_duration.lock().unwrap() = v;
        Ok(())
    }
}
