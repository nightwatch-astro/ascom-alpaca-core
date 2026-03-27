use crate::device::Device;
use crate::telescope::{AlignmentMode, EquatorialSystem, Telescope};
use crate::types::{AlpacaResult, DeviceType};

pub struct MockTelescope;

impl Device for MockTelescope {
    fn static_name(&self) -> &str { "Mock Telescope" }
    fn unique_id(&self) -> &str { "mock-tel-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Telescope }
    fn connected(&self) -> AlpacaResult<bool> { Ok(true) }
    fn set_connected(&self, _: bool) -> AlpacaResult<()> { Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock Telescope".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(3) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Telescope".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl Telescope for MockTelescope {
    fn alignment_mode(&self) -> AlpacaResult<AlignmentMode> { Ok(AlignmentMode::GermanPolar) }
    fn equatorial_system(&self) -> AlpacaResult<EquatorialSystem> { Ok(EquatorialSystem::J2000) }
    fn tracking(&self) -> AlpacaResult<bool> { Ok(false) }
    fn slewing(&self) -> AlpacaResult<bool> { Ok(false) }
    fn at_home(&self) -> AlpacaResult<bool> { Ok(false) }
    fn at_park(&self) -> AlpacaResult<bool> { Ok(true) }
    fn altitude(&self) -> AlpacaResult<f64> { Ok(45.0) }
    fn azimuth(&self) -> AlpacaResult<f64> { Ok(180.0) }
    fn right_ascension(&self) -> AlpacaResult<f64> { Ok(12.0) }
    fn declination(&self) -> AlpacaResult<f64> { Ok(45.0) }
    fn sidereal_time(&self) -> AlpacaResult<f64> { Ok(12.0) }
    fn site_elevation(&self) -> AlpacaResult<f64> { Ok(100.0) }
    fn site_latitude(&self) -> AlpacaResult<f64> { Ok(52.0) }
    fn site_longitude(&self) -> AlpacaResult<f64> { Ok(5.0) }
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
    fn unpark(&self) -> AlpacaResult<()> { Ok(()) }
    fn park(&self) -> AlpacaResult<()> { Ok(()) }
    fn abort_slew(&self) -> AlpacaResult<()> { Ok(()) }
}
