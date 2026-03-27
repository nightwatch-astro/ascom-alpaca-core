pub mod types;

pub use types::*;

use crate::device::Device;
use crate::types::{AlpacaError, AlpacaResult};

/// ASCOM Telescope device trait (~60 methods).
pub trait Telescope: Device {
    // --- Position & coordinates ---

    fn altitude(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("altitude".into()))
    }

    fn azimuth(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("azimuth".into()))
    }

    fn right_ascension(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("right_ascension".into()))
    }

    fn declination(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("declination".into()))
    }

    fn target_right_ascension(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("target_right_ascension".into()))
    }

    fn set_target_right_ascension(&self, _ra: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_target_right_ascension".into()))
    }

    fn target_declination(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("target_declination".into()))
    }

    fn set_target_declination(&self, _dec: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_target_declination".into()))
    }

    fn sidereal_time(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("sidereal_time".into()))
    }

    // --- Slewing ---

    fn slewing(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("slewing".into()))
    }

    fn slew_to_coordinates(&self, _ra: f64, _dec: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("slew_to_coordinates".into()))
    }

    fn slew_to_coordinates_async(&self, _ra: f64, _dec: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("slew_to_coordinates_async".into()))
    }

    fn slew_to_alt_az(&self, _azimuth: f64, _altitude: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("slew_to_alt_az".into()))
    }

    fn slew_to_alt_az_async(&self, _azimuth: f64, _altitude: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("slew_to_alt_az_async".into()))
    }

    fn slew_to_target(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("slew_to_target".into()))
    }

    fn slew_to_target_async(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("slew_to_target_async".into()))
    }

    fn abort_slew(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("abort_slew".into()))
    }

    fn move_axis(&self, _axis: i32, _rate: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("move_axis".into()))
    }

    fn destination_side_of_pier(&self, _ra: f64, _dec: f64) -> AlpacaResult<SideOfPier> {
        Err(AlpacaError::NotImplemented("destination_side_of_pier".into()))
    }

    // --- Tracking ---

    fn tracking(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("tracking".into()))
    }

    fn set_tracking(&self, _tracking: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_tracking".into()))
    }

    fn tracking_rate(&self) -> AlpacaResult<DriveRate> {
        Err(AlpacaError::NotImplemented("tracking_rate".into()))
    }

    fn set_tracking_rate(&self, _rate: DriveRate) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_tracking_rate".into()))
    }

    fn tracking_rates(&self) -> AlpacaResult<Vec<DriveRate>> {
        Err(AlpacaError::NotImplemented("tracking_rates".into()))
    }

    fn right_ascension_rate(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("right_ascension_rate".into()))
    }

    fn set_right_ascension_rate(&self, _rate: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_right_ascension_rate".into()))
    }

    fn declination_rate(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("declination_rate".into()))
    }

    fn set_declination_rate(&self, _rate: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_declination_rate".into()))
    }

    // --- Parking ---

    fn at_home(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("at_home".into()))
    }

    fn at_park(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("at_park".into()))
    }

    fn park(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("park".into()))
    }

    fn unpark(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("unpark".into()))
    }

    fn set_park(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_park".into()))
    }

    fn find_home(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("find_home".into()))
    }

    // --- Pulse guiding ---

    fn pulse_guide(&self, _direction: crate::camera::GuideDirection, _duration: i32) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("pulse_guide".into()))
    }

    fn is_pulse_guiding(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("is_pulse_guiding".into()))
    }

    fn guide_rate_right_ascension(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("guide_rate_right_ascension".into()))
    }

    fn set_guide_rate_right_ascension(&self, _rate: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_guide_rate_right_ascension".into()))
    }

    fn guide_rate_declination(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("guide_rate_declination".into()))
    }

    fn set_guide_rate_declination(&self, _rate: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_guide_rate_declination".into()))
    }

    // --- Side of pier ---

    fn side_of_pier(&self) -> AlpacaResult<SideOfPier> {
        Err(AlpacaError::NotImplemented("side_of_pier".into()))
    }

    fn set_side_of_pier(&self, _side: SideOfPier) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_side_of_pier".into()))
    }

    // --- Site location ---

    fn site_elevation(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("site_elevation".into()))
    }

    fn set_site_elevation(&self, _elevation: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_site_elevation".into()))
    }

    fn site_latitude(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("site_latitude".into()))
    }

    fn set_site_latitude(&self, _latitude: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_site_latitude".into()))
    }

    fn site_longitude(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("site_longitude".into()))
    }

    fn set_site_longitude(&self, _longitude: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_site_longitude".into()))
    }

    fn utc_date(&self) -> AlpacaResult<String> {
        Err(AlpacaError::NotImplemented("utc_date".into()))
    }

    fn set_utc_date(&self, _utc_date: &str) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_utc_date".into()))
    }

    // --- Axis rates ---

    fn axis_rates(&self, _axis: i32) -> AlpacaResult<Vec<AxisRates>> {
        Err(AlpacaError::NotImplemented("axis_rates".into()))
    }

    fn can_move_axis(&self, _axis: i32) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_move_axis".into()))
    }

    // --- Sync ---

    fn sync_to_coordinates(&self, _ra: f64, _dec: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("sync_to_coordinates".into()))
    }

    fn sync_to_alt_az(&self, _azimuth: f64, _altitude: f64) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("sync_to_alt_az".into()))
    }

    fn sync_to_target(&self) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("sync_to_target".into()))
    }

    // --- Capabilities ---

    fn alignment_mode(&self) -> AlpacaResult<AlignmentMode> {
        Err(AlpacaError::NotImplemented("alignment_mode".into()))
    }

    fn equatorial_system(&self) -> AlpacaResult<EquatorialSystem> {
        Err(AlpacaError::NotImplemented("equatorial_system".into()))
    }

    fn aperture_area(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("aperture_area".into()))
    }

    fn aperture_diameter(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("aperture_diameter".into()))
    }

    fn focal_length(&self) -> AlpacaResult<f64> {
        Err(AlpacaError::NotImplemented("focal_length".into()))
    }

    fn does_refraction(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("does_refraction".into()))
    }

    fn set_does_refraction(&self, _refraction: bool) -> AlpacaResult<()> {
        Err(AlpacaError::NotImplemented("set_does_refraction".into()))
    }

    fn can_find_home(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_find_home".into()))
    }

    fn can_park(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_park".into()))
    }

    fn can_pulse_guide(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_pulse_guide".into()))
    }

    fn can_set_declination_rate(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_set_declination_rate".into()))
    }

    fn can_set_guide_rates(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_set_guide_rates".into()))
    }

    fn can_set_park(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_set_park".into()))
    }

    fn can_set_pier_side(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_set_pier_side".into()))
    }

    fn can_set_right_ascension_rate(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_set_right_ascension_rate".into()))
    }

    fn can_set_tracking(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_set_tracking".into()))
    }

    fn can_slew(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_slew".into()))
    }

    fn can_slew_async(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_slew_async".into()))
    }

    fn can_slew_alt_az(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_slew_alt_az".into()))
    }

    fn can_slew_alt_az_async(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_slew_alt_az_async".into()))
    }

    fn can_sync(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_sync".into()))
    }

    fn can_sync_alt_az(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_sync_alt_az".into()))
    }

    fn can_unpark(&self) -> AlpacaResult<bool> {
        Err(AlpacaError::NotImplemented("can_unpark".into()))
    }
}
