use std::sync::Mutex;

use crate::observing_conditions::ObservingConditions;
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

/// Supported sensor property names for this mock.
const SENSORS: &[&str] = &[
    "CloudCover",
    "DewPoint",
    "Humidity",
    "Pressure",
    "RainRate",
    "SkyBrightness",
    "SkyQuality",
    "SkyTemperature",
    "StarFWHM",
    "Temperature",
    "WindDirection",
    "WindGust",
    "WindSpeed",
];

pub struct MockObservingConditions {
    connected: Mutex<bool>,
    average_period: Mutex<f64>,
}

impl Default for MockObservingConditions {
    fn default() -> Self {
        Self::new()
    }
}

impl MockObservingConditions {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
            average_period: Mutex::new(0.0),
        }
    }

    fn validate_sensor(name: &str) -> AlpacaResult<()> {
        if name.is_empty() {
            return Err(AlpacaError::InvalidValue(
                "Sensor name must not be empty".into(),
            ));
        }
        if SENSORS.iter().any(|s| s.eq_ignore_ascii_case(name)) {
            Ok(())
        } else {
            Err(AlpacaError::InvalidValue(format!("Unknown sensor: {name}")))
        }
    }
}

impl_mock_device!(MockObservingConditions,
    name: "Mock ObservingConditions",
    unique_id: "mock-oc-001",
    device_type: DeviceType::ObservingConditions,
    interface_version: 2,
    device_state: |_self: &MockObservingConditions| {
        use crate::device::common::DeviceStateBuilder;
        Ok(DeviceStateBuilder::new()
            .add("CloudCover", 20.0)
            .add("DewPoint", 7.0)
            .add("Humidity", 60.0)
            .add("Pressure", 1013.25)
            .add("RainRate", 0.0)
            .add("SkyBrightness", 21.5)
            .add("SkyQuality", 21.5)
            .add("SkyTemperature", -20.0)
            .add("StarFWHM", 2.5)
            .add("Temperature", 15.0)
            .add("WindDirection", 180.0)
            .add("WindGust", 8.0)
            .add("WindSpeed", 5.0)
            .build())
    }
);

impl ObservingConditions for MockObservingConditions {
    // All 13 weather properties with realistic mock values
    fn cloud_cover(&self) -> AlpacaResult<f64> {
        Ok(20.0)
    } // 20% cloud cover
    fn dew_point(&self) -> AlpacaResult<f64> {
        Ok(7.0)
    } // 7°C
    fn humidity(&self) -> AlpacaResult<f64> {
        Ok(60.0)
    } // 60%
    fn pressure(&self) -> AlpacaResult<f64> {
        Ok(1013.25)
    } // standard atmosphere hPa
    fn rain_rate(&self) -> AlpacaResult<f64> {
        Ok(0.0)
    } // no rain mm/hr
    fn sky_brightness(&self) -> AlpacaResult<f64> {
        Ok(21.5)
    } // mag/arcsec² (dark sky)
    fn sky_quality(&self) -> AlpacaResult<f64> {
        Ok(21.5)
    } // mag/arcsec² (SQM reading)
    fn sky_temperature(&self) -> AlpacaResult<f64> {
        Ok(-20.0)
    } // -20°C (clear sky)
    fn star_fwhm(&self) -> AlpacaResult<f64> {
        Ok(2.5)
    } // 2.5 arcsec seeing
    fn temperature(&self) -> AlpacaResult<f64> {
        Ok(15.0)
    } // 15°C ambient
    fn wind_direction(&self) -> AlpacaResult<f64> {
        Ok(180.0)
    } // south wind
    fn wind_gust(&self) -> AlpacaResult<f64> {
        Ok(8.0)
    } // 8 m/s gust
    fn wind_speed(&self) -> AlpacaResult<f64> {
        Ok(5.0)
    } // 5 m/s sustained

    fn average_period(&self) -> AlpacaResult<f64> {
        Ok(*self.average_period.lock().unwrap())
    }

    fn set_average_period(&self, hours: f64) -> AlpacaResult<()> {
        if hours < 0.0 {
            return Err(AlpacaError::InvalidValue(format!(
                "AveragePeriod must be >= 0, got {hours}"
            )));
        }
        *self.average_period.lock().unwrap() = hours;
        Ok(())
    }

    fn sensor_description(&self, property_name: &str) -> AlpacaResult<String> {
        Self::validate_sensor(property_name)?;
        Ok(format!("Mock {property_name} sensor"))
    }

    fn time_of_latest_update(&self, property_name: &str) -> AlpacaResult<f64> {
        // Empty string means "overall" time since last update per ASCOM spec
        if !property_name.is_empty() {
            Self::validate_sensor(property_name)?;
        }
        // Return 0.0 seconds since last update (just refreshed)
        Ok(0.0)
    }

    fn refresh(&self) -> AlpacaResult<()> {
        Ok(())
    }
}
