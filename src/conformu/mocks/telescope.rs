use std::sync::Mutex;

use crate::device::Device;
use crate::telescope::{AlignmentMode, EquatorialSystem, Telescope};
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

pub struct MockTelescope {
    connected: Mutex<bool>,
    at_park: Mutex<bool>,
    tracking: Mutex<bool>,
    site_elevation: Mutex<f64>,
    site_latitude: Mutex<f64>,
    site_longitude: Mutex<f64>,
    target_ra: Mutex<Option<f64>>,
    target_dec: Mutex<Option<f64>>,
}

impl MockTelescope {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
            at_park: Mutex::new(true),
            tracking: Mutex::new(false),
            site_elevation: Mutex::new(100.0),
            site_latitude: Mutex::new(52.0),
            site_longitude: Mutex::new(5.0),
            target_ra: Mutex::new(None),
            target_dec: Mutex::new(None),
        }
    }
}

impl Device for MockTelescope {
    fn static_name(&self) -> &str { "Mock Telescope" }
    fn unique_id(&self) -> &str { "mock-tel-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Telescope }
    fn connected(&self) -> AlpacaResult<bool> { Ok(*self.connected.lock().unwrap()) }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> { *self.connected.lock().unwrap() = v; Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = true; Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = false; Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock Telescope".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(3) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Telescope".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
    fn device_state(&self) -> AlpacaResult<Vec<crate::device::common::DeviceStateItem>> { Ok(vec![]) }
}

impl Telescope for MockTelescope {
    fn alignment_mode(&self) -> AlpacaResult<AlignmentMode> { Ok(AlignmentMode::GermanPolar) }
    fn equatorial_system(&self) -> AlpacaResult<EquatorialSystem> { Ok(EquatorialSystem::J2000) }

    fn tracking(&self) -> AlpacaResult<bool> { Ok(*self.tracking.lock().unwrap()) }

    fn set_tracking(&self, _tracking: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_tracking not supported — can_set_tracking is false".into()))
    }

    fn slewing(&self) -> AlpacaResult<bool> { Ok(false) }
    fn at_home(&self) -> AlpacaResult<bool> { Ok(false) }
    fn at_park(&self) -> AlpacaResult<bool> { Ok(*self.at_park.lock().unwrap()) }
    fn altitude(&self) -> AlpacaResult<f64> { Ok(45.0) }
    fn azimuth(&self) -> AlpacaResult<f64> { Ok(180.0) }
    fn right_ascension(&self) -> AlpacaResult<f64> { Ok(12.0) }
    fn declination(&self) -> AlpacaResult<f64> { Ok(45.0) }
    fn sidereal_time(&self) -> AlpacaResult<f64> { Ok(12.0) }

    fn site_elevation(&self) -> AlpacaResult<f64> { Ok(*self.site_elevation.lock().unwrap()) }
    fn set_site_elevation(&self, elevation: f64) -> AlpacaResult<()> {
        *self.site_elevation.lock().unwrap() = elevation;
        Ok(())
    }

    fn site_latitude(&self) -> AlpacaResult<f64> { Ok(*self.site_latitude.lock().unwrap()) }
    fn set_site_latitude(&self, latitude: f64) -> AlpacaResult<()> {
        *self.site_latitude.lock().unwrap() = latitude;
        Ok(())
    }

    fn site_longitude(&self) -> AlpacaResult<f64> { Ok(*self.site_longitude.lock().unwrap()) }
    fn set_site_longitude(&self, longitude: f64) -> AlpacaResult<()> {
        *self.site_longitude.lock().unwrap() = longitude;
        Ok(())
    }

    fn target_right_ascension(&self) -> AlpacaResult<f64> {
        self.target_ra.lock().unwrap().ok_or_else(|| {
            AlpacaError::ValueNotSet("TargetRightAscension has not been set".into())
        })
    }

    fn set_target_right_ascension(&self, ra: f64) -> AlpacaResult<()> {
        *self.target_ra.lock().unwrap() = Some(ra);
        Ok(())
    }

    fn target_declination(&self) -> AlpacaResult<f64> {
        self.target_dec.lock().unwrap().ok_or_else(|| {
            AlpacaError::ValueNotSet("TargetDeclination has not been set".into())
        })
    }

    fn set_target_declination(&self, dec: f64) -> AlpacaResult<()> {
        *self.target_dec.lock().unwrap() = Some(dec);
        Ok(())
    }

    fn can_find_home(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_park(&self) -> AlpacaResult<bool> { Ok(true) }
    fn can_pulse_guide(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_slew(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_slew_async(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_slew_alt_az(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_slew_alt_az_async(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_sync(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_sync_alt_az(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_unpark(&self) -> AlpacaResult<bool> { Ok(true) }
    fn can_set_tracking(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_set_park(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_set_pier_side(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_set_guide_rates(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_set_declination_rate(&self) -> AlpacaResult<bool> { Ok(false) }
    fn can_set_right_ascension_rate(&self) -> AlpacaResult<bool> { Ok(false) }
    fn does_refraction(&self) -> AlpacaResult<bool> { Ok(false) }
    fn aperture_area(&self) -> AlpacaResult<f64> { Ok(0.0269) }
    fn aperture_diameter(&self) -> AlpacaResult<f64> { Ok(0.185) }
    fn focal_length(&self) -> AlpacaResult<f64> { Ok(1.0) }
    fn is_pulse_guiding(&self) -> AlpacaResult<bool> { Ok(false) }

    fn unpark(&self) -> AlpacaResult<()> {
        *self.at_park.lock().unwrap() = false;
        Ok(())
    }

    fn park(&self) -> AlpacaResult<()> {
        *self.at_park.lock().unwrap() = true;
        *self.tracking.lock().unwrap() = false;
        Ok(())
    }

    fn abort_slew(&self) -> AlpacaResult<()> { Ok(()) }
}
