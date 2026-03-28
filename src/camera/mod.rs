pub mod image;
pub mod types;

pub use image::{ImageArrayResponse, ImageData};
pub use types::*;

use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// ASCOM Camera device trait (~55 methods).
///
/// The most complex ASCOM device type. Capabilities are organized in groups:
///
/// - **Exposure**: `start_exposure`, `stop_exposure`, `abort_exposure`, `image_ready`, `image_array`
/// - **Sensor**: `camera_xsize/ysize`, `pixel_size_x/y`, `sensor_type`, `bayer_offset_x/y`
/// - **Binning**: `bin_x/y`, `max_bin_x/y`, `can_asymmetric_bin`
/// - **Subframe**: `start_x/y`, `num_x/y`
/// - **Cooling**: `cooler_on`, `set_ccd_temperature`, `cooler_power` (gated by `can_set_ccd_temperature`)
/// - **Gain**: Two mutually exclusive modes:
///   - *Numeric*: `gain`, `gain_min`, `gain_max` work; `gains()` returns `NotImplemented`
///   - *Named*: `gains()` returns a name list; `gain_min/gain_max` return `NotImplemented`
/// - **Offset**: Same two mutually exclusive modes as gain
/// - **Pulse guide**: `pulse_guide`, `is_pulse_guiding` (gated by `can_pulse_guide`)
/// - **Readout**: `readout_mode`, `readout_modes`, `fast_readout` (gated by `can_fast_readout`)
///
/// All methods default to `NotImplemented` — only override what your hardware supports.
///
/// # Example
///
/// ```rust
/// use ascom_alpaca_core::prelude::*;
/// use ascom_alpaca_core::camera::{Camera, CameraState, ImageData};
///
/// # struct MyCam;
/// # impl Device for MyCam {
/// #     fn static_name(&self) -> &str { "Cam" }
/// #     fn unique_id(&self) -> &str { "c-001" }
/// #     fn device_type(&self) -> DeviceType { DeviceType::Camera }
/// # }
/// impl Camera for MyCam {
///     fn camera_xsize(&self) -> AlpacaResult<i32> { Ok(4656) }
///     fn camera_ysize(&self) -> AlpacaResult<i32> { Ok(3520) }
///     fn sensor_type(&self) -> AlpacaResult<ascom_alpaca_core::camera::SensorType> {
///         Ok(ascom_alpaca_core::camera::SensorType::Monochrome)
///     }
///     fn start_exposure(&self, duration: f64, _light: bool) -> AlpacaResult<()> {
///         // start your hardware exposure
///         Ok(())
///     }
///     fn can_set_ccd_temperature(&self) -> AlpacaResult<bool> { Ok(false) }
///     fn can_pulse_guide(&self) -> AlpacaResult<bool> { Ok(false) }
/// }
/// ```
pub trait Camera: Device {
    // --- Exposure control ---

    fn start_exposure(&self, _duration: f64, _light: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("start_exposure".into()))
    }

    fn stop_exposure(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("stop_exposure".into()))
    }

    fn abort_exposure(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("abort_exposure".into()))
    }

    fn image_ready(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("image_ready".into()))
    }

    fn image_array(&self) -> AlpacaResult<ImageData> {
        Err(AlpacaError::NotImplemented("image_array".into()))
    }

    fn last_exposure_duration(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("last_exposure_duration".into()))
    }

    fn last_exposure_start_time(&self) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented(
            "last_exposure_start_time".into(),
        ))
    }

    fn exposure_min(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("exposure_min".into()))
    }

    fn exposure_max(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("exposure_max".into()))
    }

    fn exposure_resolution(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("exposure_resolution".into()))
    }

    // --- Camera state ---

    fn camera_state(&self) -> AlpacaResult<CameraState> {
        Err(AlpacaError::NotImplemented("camera_state".into()))
    }

    fn percent_completed(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("percent_completed".into()))
    }

    fn camera_xsize(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("camera_xsize".into()))
    }

    fn camera_ysize(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("camera_ysize".into()))
    }

    fn max_adu(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("max_adu".into()))
    }

    // --- Binning ---

    fn bin_x(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("bin_x".into()))
    }

    fn set_bin_x(&self, _bin_x: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_bin_x".into()))
    }

    fn bin_y(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("bin_y".into()))
    }

    fn set_bin_y(&self, _bin_y: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_bin_y".into()))
    }

    fn max_bin_x(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("max_bin_x".into()))
    }

    fn max_bin_y(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("max_bin_y".into()))
    }

    fn can_asymmetric_bin(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_asymmetric_bin".into()))
    }

    // --- Subframe ---

    fn start_x(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("start_x".into()))
    }

    fn set_start_x(&self, _start_x: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_start_x".into()))
    }

    fn start_y(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("start_y".into()))
    }

    fn set_start_y(&self, _start_y: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_start_y".into()))
    }

    fn num_x(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("num_x".into()))
    }

    fn set_num_x(&self, _num_x: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_num_x".into()))
    }

    fn num_y(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("num_y".into()))
    }

    fn set_num_y(&self, _num_y: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_num_y".into()))
    }

    // --- Gain & Offset ---

    fn gain(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("gain".into()))
    }

    fn set_gain(&self, _gain: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_gain".into()))
    }

    fn gain_min(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("gain_min".into()))
    }

    fn gain_max(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("gain_max".into()))
    }

    fn gains(&self) -> AlpacaResult<Vec<String>> {
        Err(AlpacaError::NotImplemented("gains".into()))
    }

    fn offset(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("offset".into()))
    }

    fn set_offset(&self, _offset: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_offset".into()))
    }

    fn offset_min(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("offset_min".into()))
    }

    fn offset_max(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("offset_max".into()))
    }

    fn offsets(&self) -> AlpacaResult<Vec<String>> {
        Err(AlpacaError::NotImplemented("offsets".into()))
    }

    // --- Cooler ---

    fn cooler_on(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("cooler_on".into()))
    }

    fn set_cooler_on(&self, _on: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_cooler_on".into()))
    }

    fn cooler_power(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("cooler_power".into()))
    }

    fn ccd_temperature(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("ccd_temperature".into()))
    }

    fn heat_sink_temperature(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("heat_sink_temperature".into()))
    }

    /// Returns the target CCD temperature (ASCOM property: `SetCCDTemperature`).
    fn set_ccd_temperature(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("set_ccd_temperature".into()))
    }

    /// Sets the target CCD temperature. The double "set" is intentional —
    /// `SetCCDTemperature` is the ASCOM property name (the target temp),
    /// so its setter becomes `set_set_ccd_temperature`.
    fn set_set_ccd_temperature(&self, _temp: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented(
            "set_set_ccd_temperature".into(),
        ))
    }

    fn can_set_ccd_temperature(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented(
            "can_set_ccd_temperature".into(),
        ))
    }

    fn can_get_cooler_power(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_get_cooler_power".into()))
    }

    // --- Pulse guiding ---

    fn can_pulse_guide(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_pulse_guide".into()))
    }

    fn is_pulse_guiding(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("is_pulse_guiding".into()))
    }

    fn pulse_guide(&self, _direction: GuideDirection, _duration: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("pulse_guide".into()))
    }

    // --- Sensor info ---

    fn sensor_name(&self) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("sensor_name".into()))
    }

    fn sensor_type(&self) -> AlpacaResult<SensorType> {
        Err(AlpacaError::NotImplemented("sensor_type".into()))
    }

    fn pixel_size_x(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("pixel_size_x".into()))
    }

    fn pixel_size_y(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("pixel_size_y".into()))
    }

    fn electrons_per_adu(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("electrons_per_adu".into()))
    }

    fn full_well_capacity(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("full_well_capacity".into()))
    }

    fn has_shutter(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("has_shutter".into()))
    }

    fn bayer_offset_x(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("bayer_offset_x".into()))
    }

    fn bayer_offset_y(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("bayer_offset_y".into()))
    }

    // --- Readout modes ---

    fn readout_mode(&self) -> AlpacaResult<i32> {
        Err(AlpacaError::NotImplemented("readout_mode".into()))
    }

    fn set_readout_mode(&self, _mode: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_readout_mode".into()))
    }

    fn readout_modes(&self) -> AlpacaResult<Vec<String>> {
        Err(AlpacaError::NotImplemented("readout_modes".into()))
    }

    // --- Fast readout ---

    fn can_fast_readout(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_fast_readout".into()))
    }

    fn fast_readout(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("fast_readout".into()))
    }

    fn set_fast_readout(&self, _fast: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_fast_readout".into()))
    }

    // --- Sub exposure ---

    fn sub_exposure_duration(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("sub_exposure_duration".into()))
    }

    fn set_sub_exposure_duration(&self, _duration: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented(
            "set_sub_exposure_duration".into(),
        ))
    }

    // --- Capabilities ---

    fn can_abort_exposure(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_abort_exposure".into()))
    }

    fn can_stop_exposure(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_stop_exposure".into()))
    }
}
