use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// ASCOM ObservingConditions device trait.
pub trait ObservingConditions: Device {
    fn cloud_cover(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("cloud_cover".into()))
    }

    fn dew_point(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("dew_point".into()))
    }

    fn humidity(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("humidity".into()))
    }

    fn pressure(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("pressure".into()))
    }

    fn rain_rate(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("rain_rate".into()))
    }

    fn sky_brightness(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("sky_brightness".into()))
    }

    fn sky_quality(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("sky_quality".into()))
    }

    fn sky_temperature(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("sky_temperature".into()))
    }

    fn star_fwhm(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("star_fwhm".into()))
    }

    fn temperature(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("temperature".into()))
    }

    fn wind_direction(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("wind_direction".into()))
    }

    fn wind_gust(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("wind_gust".into()))
    }

    fn wind_speed(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("wind_speed".into()))
    }

    /// Returns the time period over which observations are averaged (hours).
    fn average_period(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("average_period".into()))
    }

    /// Sets the averaging period (hours).
    fn set_average_period(&self, _hours: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_average_period".into()))
    }

    /// Returns a description of the sensor for the specified property.
    fn sensor_description(&self, _property_name: &str) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("sensor_description".into()))
    }

    /// Returns the time since the sensor was last updated (seconds).
    fn time_of_latest_update(&self, _property_name: &str) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("time_of_latest_update".into()))
    }

    /// Refreshes sensor values from the hardware.
    fn refresh(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("refresh".into()))
    }
}
