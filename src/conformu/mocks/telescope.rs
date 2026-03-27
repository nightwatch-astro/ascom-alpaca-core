use std::sync::Mutex;
use std::time::Instant;

use crate::device::Device;
use crate::telescope::{
    AlignmentMode, AxisRates, DriveRate, EquatorialSystem, SideOfPier, Telescope,
};
use crate::types::{AlpacaError, AlpacaResult, DeviceType, GuideDirection};

pub struct MockTelescope {
    connected: Mutex<bool>,
    at_park: Mutex<bool>,
    at_home: Mutex<bool>,
    tracking: Mutex<bool>,
    tracking_rate: Mutex<DriveRate>,
    ra: Mutex<f64>,
    dec: Mutex<f64>,
    target_ra: Mutex<Option<f64>>,
    target_dec: Mutex<Option<f64>>,
    altitude: Mutex<f64>,
    azimuth: Mutex<f64>,
    slew_start: Mutex<Option<Instant>>,
    site_elevation: Mutex<f64>,
    site_latitude: Mutex<f64>,
    site_longitude: Mutex<f64>,
    guide_rate_ra: Mutex<f64>,
    guide_rate_dec: Mutex<f64>,
    ra_rate: Mutex<f64>,
    dec_rate: Mutex<f64>,
    rate_base_ra: Mutex<f64>,
    rate_base_dec: Mutex<f64>,
    rate_set_at: Mutex<Option<Instant>>,
    slew_settle_time: Mutex<i32>,
    does_refraction: Mutex<bool>,
    pulse_guiding: Mutex<bool>,
    move_axis_active: Mutex<bool>,
    pulse_guide_start: Mutex<Option<Instant>>,
    pulse_guide_duration_ms: Mutex<i32>,
}

impl MockTelescope {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
            at_park: Mutex::new(true),
            at_home: Mutex::new(false),
            tracking: Mutex::new(false),
            tracking_rate: Mutex::new(DriveRate::Sidereal),
            ra: Mutex::new(12.0),
            dec: Mutex::new(45.0),
            target_ra: Mutex::new(None),
            target_dec: Mutex::new(None),
            altitude: Mutex::new(45.0),
            azimuth: Mutex::new(180.0),
            slew_start: Mutex::new(None),
            site_elevation: Mutex::new(100.0),
            site_latitude: Mutex::new(52.0),
            site_longitude: Mutex::new(5.0),
            guide_rate_ra: Mutex::new(0.5),
            guide_rate_dec: Mutex::new(0.5),
            ra_rate: Mutex::new(0.0),
            dec_rate: Mutex::new(0.0),
            rate_base_ra: Mutex::new(12.0),
            rate_base_dec: Mutex::new(45.0),
            rate_set_at: Mutex::new(None),
            slew_settle_time: Mutex::new(0),
            does_refraction: Mutex::new(false),
            pulse_guiding: Mutex::new(false),
            move_axis_active: Mutex::new(false),
            pulse_guide_start: Mutex::new(None),
            pulse_guide_duration_ms: Mutex::new(0),
        }
    }

    /// Check if a slew has completed (>4000ms elapsed) and update position.
    fn check_slew_complete(&self) {
        let mut slew = self.slew_start.lock().unwrap();
        if let Some(start) = *slew {
            if start.elapsed().as_millis() >= 4000 {
                // Slew complete — update current position from targets
                if let Some(tra) = *self.target_ra.lock().unwrap() {
                    *self.ra.lock().unwrap() = tra;
                    *self.rate_base_ra.lock().unwrap() = tra;
                }
                if let Some(tdec) = *self.target_dec.lock().unwrap() {
                    *self.dec.lock().unwrap() = tdec;
                    *self.rate_base_dec.lock().unwrap() = tdec;
                }
                // Reset rate tracking origin if rates are active
                *self.rate_set_at.lock().unwrap() =
                    if *self.ra_rate.lock().unwrap() != 0.0
                        || *self.dec_rate.lock().unwrap() != 0.0
                    {
                        Some(Instant::now())
                    } else {
                        None
                    };
                *slew = None;
            }
        }
    }

    /// Compute side of pier from hour angle.
    ///
    /// ASCOM SideOfPier for a GEM:
    /// - pierEast (0) = "Normal" — counterweight down, OTA on east side, looking west.
    ///   This is the normal tracking position when the object is WEST of the meridian
    ///   (positive HA, object has crossed the meridian).
    /// - pierWest (1) = "Through the pole" — OTA on west side, looking east.
    ///   Object is EAST of the meridian (negative HA, before crossing).
    fn compute_side_of_pier(&self, ra: f64, lst: f64) -> AlpacaResult<SideOfPier> {
        let ha = lst - ra;
        // Normalize HA to [-12, +12)
        let ha_norm = ((ha % 24.0) + 36.0) % 24.0 - 12.0;
        if ha_norm >= 0.0 {
            // Positive HA: object west of meridian → normal tracking → pierEast
            Ok(SideOfPier::East)
        } else {
            // Negative HA: object east of meridian → through the pole → pierWest
            Ok(SideOfPier::West)
        }
    }

    fn check_not_parked(&self) -> AlpacaResult<()> {
        if *self.at_park.lock().unwrap() {
            Err(AlpacaError::InvalidWhileParked(
                "Operation not allowed while parked".into(),
            ))
        } else {
            Ok(())
        }
    }
}

impl Device for MockTelescope {
    fn static_name(&self) -> &str {
        "Mock Telescope"
    }
    fn unique_id(&self) -> &str {
        "mock-tel-001"
    }
    fn device_type(&self) -> DeviceType {
        DeviceType::Telescope
    }
    fn connected(&self) -> AlpacaResult<bool> {
        Ok(*self.connected.lock().unwrap())
    }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> {
        *self.connected.lock().unwrap() = v;
        Ok(())
    }
    fn connecting(&self) -> AlpacaResult<bool> {
        Ok(false)
    }
    fn connect(&self) -> AlpacaResult<()> {
        *self.connected.lock().unwrap() = true;
        Ok(())
    }
    fn disconnect(&self) -> AlpacaResult<()> {
        *self.connected.lock().unwrap() = false;
        Ok(())
    }
    fn description(&self) -> AlpacaResult<String> {
        Ok("Mock Telescope".into())
    }
    fn driver_info(&self) -> AlpacaResult<String> {
        Ok("ascom-alpaca-core mock".into())
    }
    fn driver_version(&self) -> AlpacaResult<String> {
        Ok(env!("CARGO_PKG_VERSION").into())
    }
    fn interface_version(&self) -> AlpacaResult<i32> {
        Ok(4)
    }
    fn name(&self) -> AlpacaResult<String> {
        Ok("Mock Telescope".into())
    }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> {
        Ok(vec![])
    }
    fn device_state(&self) -> AlpacaResult<Vec<crate::device::common::DeviceStateItem>> {
        use crate::device::common::DeviceStateItem;
        self.check_slew_complete();
        let slewing = self.slew_start.lock().unwrap().is_some()
            || *self.move_axis_active.lock().unwrap();
        Ok(vec![
            DeviceStateItem { name: "Altitude".into(), value: serde_json::json!(*self.altitude.lock().unwrap()) },
            DeviceStateItem { name: "AtHome".into(), value: serde_json::json!(*self.at_home.lock().unwrap()) },
            DeviceStateItem { name: "AtPark".into(), value: serde_json::json!(*self.at_park.lock().unwrap()) },
            DeviceStateItem { name: "Azimuth".into(), value: serde_json::json!(*self.azimuth.lock().unwrap()) },
            DeviceStateItem { name: "Declination".into(), value: serde_json::json!(*self.dec.lock().unwrap()) },
            DeviceStateItem { name: "IsPulseGuiding".into(), value: serde_json::json!(*self.pulse_guiding.lock().unwrap()) },
            DeviceStateItem { name: "RightAscension".into(), value: serde_json::json!(*self.ra.lock().unwrap()) },
            DeviceStateItem { name: "SideOfPier".into(), value: serde_json::json!(0) },
            DeviceStateItem { name: "SiderealTime".into(), value: serde_json::json!(self.sidereal_time().unwrap_or(0.0)) },
            DeviceStateItem { name: "Slewing".into(), value: serde_json::json!(slewing) },
            DeviceStateItem { name: "Tracking".into(), value: serde_json::json!(*self.tracking.lock().unwrap()) },
            DeviceStateItem { name: "UTCDate".into(), value: serde_json::json!(self.utc_date().unwrap_or_default()) },
        ])
    }
}

impl Telescope for MockTelescope {
    // --- Position & coordinates ---

    fn altitude(&self) -> AlpacaResult<f64> {
        self.check_slew_complete();
        Ok(*self.altitude.lock().unwrap())
    }

    fn azimuth(&self) -> AlpacaResult<f64> {
        self.check_slew_complete();
        Ok(*self.azimuth.lock().unwrap())
    }

    fn right_ascension(&self) -> AlpacaResult<f64> {
        self.check_slew_complete();
        let base = *self.rate_base_ra.lock().unwrap();
        let rate = *self.ra_rate.lock().unwrap();
        let ra = if let Some(set_at) = *self.rate_set_at.lock().unwrap() {
            let elapsed = set_at.elapsed().as_secs_f64();
            // ra_rate is in seconds of RA per sidereal second, RA is in hours
            // 1 RA second = 1/3600 RA hour
            base + rate * elapsed / 3600.0
        } else {
            *self.ra.lock().unwrap()
        };
        // Sidereal tracking does NOT change reported RA — the mount compensates
        // for Earth's rotation, keeping the telescope on the same celestial coords.
        // Only RightAscensionRate (offset from sidereal) causes RA drift.
        // Normalize to [0, 24)
        Ok(((ra % 24.0) + 24.0) % 24.0)
    }

    fn declination(&self) -> AlpacaResult<f64> {
        self.check_slew_complete();
        let base = *self.rate_base_dec.lock().unwrap();
        let rate = *self.dec_rate.lock().unwrap();
        if let Some(set_at) = *self.rate_set_at.lock().unwrap() {
            let elapsed = set_at.elapsed().as_secs_f64();
            // dec_rate is in arcseconds/second, declination is in degrees
            Ok(base + rate * elapsed / 3600.0)
        } else {
            Ok(*self.dec.lock().unwrap())
        }
    }

    fn target_right_ascension(&self) -> AlpacaResult<f64> {
        self.target_ra.lock().unwrap().ok_or_else(|| {
            AlpacaError::ValueNotSet("TargetRightAscension has not been set".into())
        })
    }

    fn set_target_right_ascension(&self, ra: f64) -> AlpacaResult<()> {
        if !(0.0..24.0).contains(&ra) {
            return Err(AlpacaError::InvalidValue(format!(
                "RightAscension {ra} out of range (0 <= RA < 24)"
            )));
        }
        *self.target_ra.lock().unwrap() = Some(ra);
        Ok(())
    }

    fn target_declination(&self) -> AlpacaResult<f64> {
        self.target_dec.lock().unwrap().ok_or_else(|| {
            AlpacaError::ValueNotSet("TargetDeclination has not been set".into())
        })
    }

    fn set_target_declination(&self, dec: f64) -> AlpacaResult<()> {
        if !(-90.0..=90.0).contains(&dec) {
            return Err(AlpacaError::InvalidValue(format!(
                "Declination {dec} out of range (-90 <= Dec <= 90)"
            )));
        }
        *self.target_dec.lock().unwrap() = Some(dec);
        Ok(())
    }

    fn sidereal_time(&self) -> AlpacaResult<f64> {
        // Approximate Local Sidereal Time from UTC
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        // Julian Date from Unix time
        let jd = secs / 86400.0 + 2440587.5;
        // Greenwich Mean Sidereal Time (hours)
        let t = (jd - 2451545.0) / 36525.0;
        let gmst = 280.46061837 + 360.98564736629 * (jd - 2451545.0) + 0.000387933 * t * t;
        let gmst_hours = (gmst % 360.0 + 360.0) % 360.0 / 15.0;
        // Add site longitude (degrees -> hours)
        let lon = *self.site_longitude.lock().unwrap();
        let lst = (gmst_hours + lon / 15.0 + 24.0) % 24.0;
        Ok(lst)
    }

    // --- Slewing ---

    fn slewing(&self) -> AlpacaResult<bool> {
        self.check_slew_complete();
        let slewing = self.slew_start.lock().unwrap().is_some();
        let moving = *self.move_axis_active.lock().unwrap();
        Ok(slewing || moving)
    }

    fn slew_to_coordinates(&self, ra: f64, dec: f64) -> AlpacaResult<()> {
        self.check_not_parked()?;
        if !(0.0..24.0).contains(&ra) {
            return Err(AlpacaError::InvalidValue(format!(
                "RightAscension {ra} out of range (0 <= RA < 24)"
            )));
        }
        if !(-90.0..=90.0).contains(&dec) {
            return Err(AlpacaError::InvalidValue(format!(
                "Declination {dec} out of range (-90 <= Dec <= 90)"
            )));
        }
        *self.target_ra.lock().unwrap() = Some(ra);
        *self.target_dec.lock().unwrap() = Some(dec);
        // "Synchronous" slew — set position immediately for single-threaded harness
        *self.ra.lock().unwrap() = ra;
        *self.dec.lock().unwrap() = dec;
        *self.rate_base_ra.lock().unwrap() = ra;
        *self.rate_base_dec.lock().unwrap() = dec;
        Ok(())
    }

    fn slew_to_coordinates_async(&self, ra: f64, dec: f64) -> AlpacaResult<()> {
        self.check_not_parked()?;
        if !(0.0..24.0).contains(&ra) {
            return Err(AlpacaError::InvalidValue(format!(
                "RightAscension {ra} out of range (0 <= RA < 24)"
            )));
        }
        if !(-90.0..=90.0).contains(&dec) {
            return Err(AlpacaError::InvalidValue(format!(
                "Declination {dec} out of range (-90 <= Dec <= 90)"
            )));
        }
        *self.target_ra.lock().unwrap() = Some(ra);
        *self.target_dec.lock().unwrap() = Some(dec);
        *self.slew_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn slew_to_alt_az(&self, azimuth: f64, altitude: f64) -> AlpacaResult<()> {
        self.check_not_parked()?;
        if !(0.0..=360.0).contains(&azimuth) {
            return Err(AlpacaError::InvalidValue(format!(
                "Azimuth {azimuth} out of range (0 to 360 degrees)"
            )));
        }
        if !(0.0..=90.0).contains(&altitude) {
            return Err(AlpacaError::InvalidValue(format!(
                "Altitude {altitude} out of range (0 to 90 degrees)"
            )));
        }
        *self.azimuth.lock().unwrap() = azimuth;
        *self.altitude.lock().unwrap() = altitude;
        Ok(())
    }

    fn slew_to_alt_az_async(&self, azimuth: f64, altitude: f64) -> AlpacaResult<()> {
        self.check_not_parked()?;
        if !(0.0..=360.0).contains(&azimuth) {
            return Err(AlpacaError::InvalidValue(format!(
                "Azimuth {azimuth} out of range (0 to 360 degrees)"
            )));
        }
        if !(0.0..=90.0).contains(&altitude) {
            return Err(AlpacaError::InvalidValue(format!(
                "Altitude {altitude} out of range (0 to 90 degrees)"
            )));
        }
        // Store target alt/az and start slew timer
        *self.azimuth.lock().unwrap() = azimuth;
        *self.altitude.lock().unwrap() = altitude;
        *self.slew_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn slew_to_target(&self) -> AlpacaResult<()> {
        self.check_not_parked()?;
        let ra = self.target_ra.lock().unwrap().ok_or_else(|| {
            AlpacaError::ValueNotSet("TargetRightAscension has not been set".into())
        })?;
        let dec = self.target_dec.lock().unwrap().ok_or_else(|| {
            AlpacaError::ValueNotSet("TargetDeclination has not been set".into())
        })?;
        *self.ra.lock().unwrap() = ra;
        *self.dec.lock().unwrap() = dec;
        *self.rate_base_ra.lock().unwrap() = ra;
        *self.rate_base_dec.lock().unwrap() = dec;
        Ok(())
    }

    fn slew_to_target_async(&self) -> AlpacaResult<()> {
        self.check_not_parked()?;
        let _ra = self.target_ra.lock().unwrap().ok_or_else(|| {
            AlpacaError::ValueNotSet("TargetRightAscension has not been set".into())
        })?;
        let _dec = self.target_dec.lock().unwrap().ok_or_else(|| {
            AlpacaError::ValueNotSet("TargetDeclination has not been set".into())
        })?;
        *self.slew_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn abort_slew(&self) -> AlpacaResult<()> {
        self.check_not_parked()?;
        *self.slew_start.lock().unwrap() = None;
        Ok(())
    }

    fn move_axis(&self, axis: i32, rate: f64) -> AlpacaResult<()> {
        self.check_not_parked()?;
        if axis < 0 || axis > 2 {
            return Err(AlpacaError::InvalidValue(format!(
                "Axis must be 0, 1, or 2, got {axis}"
            )));
        }
        if axis == 2 {
            return Err(AlpacaError::InvalidValue(
                "Tertiary axis not supported".into(),
            ));
        }
        if rate != 0.0 && rate.abs() > 5.0 {
            return Err(AlpacaError::InvalidValue(format!(
                "Rate {rate} exceeds maximum axis rate of 5.0"
            )));
        }
        *self.move_axis_active.lock().unwrap() = rate != 0.0;
        Ok(())
    }

    fn destination_side_of_pier(&self, ra: f64, dec: f64) -> AlpacaResult<SideOfPier> {
        if !(0.0..24.0).contains(&ra) {
            return Err(AlpacaError::InvalidValue(format!(
                "RightAscension {ra} out of range (0 <= RA < 24)"
            )));
        }
        if !(-90.0..=90.0).contains(&dec) {
            return Err(AlpacaError::InvalidValue(format!(
                "Declination {dec} out of range (-90 <= Dec <= 90)"
            )));
        }
        let lst = self.sidereal_time()?;
        self.compute_side_of_pier(ra, lst)
    }

    // --- Tracking ---

    fn tracking(&self) -> AlpacaResult<bool> {
        Ok(*self.tracking.lock().unwrap())
    }

    fn set_tracking(&self, tracking: bool) -> AlpacaResult<()> {
        // V4: setting tracking=false while parked must succeed (it's already not tracking)
        if !tracking && *self.at_park.lock().unwrap() {
            return Ok(());
        }
        self.check_not_parked()?;
        *self.tracking.lock().unwrap() = tracking;
        Ok(())
    }

    fn tracking_rate(&self) -> AlpacaResult<DriveRate> {
        Ok(*self.tracking_rate.lock().unwrap())
    }

    fn set_tracking_rate(&self, rate: DriveRate) -> AlpacaResult<()> {
        *self.tracking_rate.lock().unwrap() = rate;
        Ok(())
    }

    fn tracking_rates(&self) -> AlpacaResult<Vec<DriveRate>> {
        Ok(vec![
            DriveRate::Sidereal,
            DriveRate::Lunar,
            DriveRate::Solar,
            DriveRate::King,
        ])
    }

    fn right_ascension_rate(&self) -> AlpacaResult<f64> {
        Ok(*self.ra_rate.lock().unwrap())
    }

    fn set_right_ascension_rate(&self, rate: f64) -> AlpacaResult<()> {
        // RA/Dec rate offsets only valid when tracking at sidereal rate
        let current_rate = *self.tracking_rate.lock().unwrap();
        if current_rate != DriveRate::Sidereal {
            return Err(AlpacaError::InvalidOperationException(
                format!("Cannot set RightAscensionRate when tracking rate is {:?} (must be Sidereal)", current_rate)
            ));
        }
        self.check_slew_complete();
        // Snapshot current effective position before changing rates
        // (inline the drift calculation to avoid re-entrant locking)
        let current_ra = {
            let base = *self.rate_base_ra.lock().unwrap();
            let old_rate = *self.ra_rate.lock().unwrap();
            if let Some(set_at) = *self.rate_set_at.lock().unwrap() {
                base + old_rate * set_at.elapsed().as_secs_f64() / 3600.0
            } else {
                *self.ra.lock().unwrap()
            }
        };
        let current_dec = {
            let base = *self.rate_base_dec.lock().unwrap();
            let old_rate = *self.dec_rate.lock().unwrap();
            if let Some(set_at) = *self.rate_set_at.lock().unwrap() {
                base + old_rate * set_at.elapsed().as_secs_f64() / 3600.0
            } else {
                *self.dec.lock().unwrap()
            }
        };
        *self.rate_base_ra.lock().unwrap() = current_ra;
        *self.rate_base_dec.lock().unwrap() = current_dec;
        *self.rate_set_at.lock().unwrap() = Some(Instant::now());
        *self.ra_rate.lock().unwrap() = rate;
        Ok(())
    }

    fn declination_rate(&self) -> AlpacaResult<f64> {
        Ok(*self.dec_rate.lock().unwrap())
    }

    fn set_declination_rate(&self, rate: f64) -> AlpacaResult<()> {
        let current_rate = *self.tracking_rate.lock().unwrap();
        if current_rate != DriveRate::Sidereal {
            return Err(AlpacaError::InvalidOperationException(
                format!("Cannot set DeclinationRate when tracking rate is {:?} (must be Sidereal)", current_rate)
            ));
        }
        self.check_slew_complete();
        // Snapshot current effective position before changing rates
        let current_ra = {
            let base = *self.rate_base_ra.lock().unwrap();
            let old_rate = *self.ra_rate.lock().unwrap();
            if let Some(set_at) = *self.rate_set_at.lock().unwrap() {
                base + old_rate * set_at.elapsed().as_secs_f64() / 3600.0
            } else {
                *self.ra.lock().unwrap()
            }
        };
        let current_dec = {
            let base = *self.rate_base_dec.lock().unwrap();
            let old_rate = *self.dec_rate.lock().unwrap();
            if let Some(set_at) = *self.rate_set_at.lock().unwrap() {
                base + old_rate * set_at.elapsed().as_secs_f64() / 3600.0
            } else {
                *self.dec.lock().unwrap()
            }
        };
        *self.rate_base_ra.lock().unwrap() = current_ra;
        *self.rate_base_dec.lock().unwrap() = current_dec;
        *self.rate_set_at.lock().unwrap() = Some(Instant::now());
        *self.dec_rate.lock().unwrap() = rate;
        Ok(())
    }

    // --- Parking ---

    fn at_home(&self) -> AlpacaResult<bool> {
        Ok(*self.at_home.lock().unwrap())
    }

    fn at_park(&self) -> AlpacaResult<bool> {
        Ok(*self.at_park.lock().unwrap())
    }

    fn park(&self) -> AlpacaResult<()> {
        *self.at_park.lock().unwrap() = true;
        *self.tracking.lock().unwrap() = false;
        Ok(())
    }

    fn unpark(&self) -> AlpacaResult<()> {
        *self.at_park.lock().unwrap() = false;
        Ok(())
    }

    fn set_park(&self) -> AlpacaResult<()> {
        Ok(())
    }

    fn find_home(&self) -> AlpacaResult<()> {
        self.check_not_parked()?;
        *self.at_home.lock().unwrap() = true;
        *self.at_park.lock().unwrap() = false;
        *self.tracking.lock().unwrap() = false;
        Ok(())
    }

    // --- Pulse guiding ---

    fn pulse_guide(&self, direction: GuideDirection, duration: i32) -> AlpacaResult<()> {
        self.check_not_parked()?;
        // ASCOM guide rates are per sidereal second, but duration is in solar milliseconds.
        // Convert solar time to sidereal time: sidereal_sec = solar_sec * (86400 / 86164.0905)
        let solar_secs = duration as f64 / 1000.0;
        let sidereal_secs = solar_secs * (86400.0 / 86164.0905);
        let guide_ra = *self.guide_rate_ra.lock().unwrap();
        let guide_dec = *self.guide_rate_dec.lock().unwrap();

        // RA guide rate operates in sidereal time; Dec guide rate operates in solar time.
        // ConformU expects RA corrections to account for the solar-to-sidereal time ratio.
        match direction {
            GuideDirection::North => {
                *self.dec.lock().unwrap() += guide_dec * solar_secs;
                *self.rate_base_dec.lock().unwrap() += guide_dec * solar_secs;
            }
            GuideDirection::South => {
                *self.dec.lock().unwrap() -= guide_dec * solar_secs;
                *self.rate_base_dec.lock().unwrap() -= guide_dec * solar_secs;
            }
            GuideDirection::East => {
                // RA is in hours, guide_rate is degrees/sidereal sec: degrees→hours (÷15)
                *self.ra.lock().unwrap() += guide_ra * sidereal_secs / 15.0;
                *self.rate_base_ra.lock().unwrap() += guide_ra * sidereal_secs / 15.0;
            }
            GuideDirection::West => {
                *self.ra.lock().unwrap() -= guide_ra * sidereal_secs / 15.0;
                *self.rate_base_ra.lock().unwrap() -= guide_ra * sidereal_secs / 15.0;
            }
        }

        *self.pulse_guiding.lock().unwrap() = true;
        *self.pulse_guide_start.lock().unwrap() = Some(Instant::now());
        *self.pulse_guide_duration_ms.lock().unwrap() = duration;
        Ok(())
    }

    fn is_pulse_guiding(&self) -> AlpacaResult<bool> {
        let start = *self.pulse_guide_start.lock().unwrap();
        if let Some(s) = start {
            let duration_ms = *self.pulse_guide_duration_ms.lock().unwrap();
            if s.elapsed().as_millis() < duration_ms as u128 {
                return Ok(true);
            }
            *self.pulse_guiding.lock().unwrap() = false;
            *self.pulse_guide_start.lock().unwrap() = None;
        }
        Ok(false)
    }

    fn guide_rate_right_ascension(&self) -> AlpacaResult<f64> {
        Ok(*self.guide_rate_ra.lock().unwrap())
    }

    fn set_guide_rate_right_ascension(&self, rate: f64) -> AlpacaResult<()> {
        *self.guide_rate_ra.lock().unwrap() = rate;
        Ok(())
    }

    fn guide_rate_declination(&self) -> AlpacaResult<f64> {
        Ok(*self.guide_rate_dec.lock().unwrap())
    }

    fn set_guide_rate_declination(&self, rate: f64) -> AlpacaResult<()> {
        *self.guide_rate_dec.lock().unwrap() = rate;
        Ok(())
    }

    // --- Side of pier ---

    fn side_of_pier(&self) -> AlpacaResult<SideOfPier> {
        let ra = self.right_ascension()?;
        let lst = self.sidereal_time()?;
        self.compute_side_of_pier(ra, lst)
    }

    fn set_side_of_pier(&self, _side: SideOfPier) -> AlpacaResult<()> {
        self.check_not_parked()?;
        // For a GEM, setting the side of pier triggers a meridian flip.
        // The telescope slews to point at the same RA/Dec from the other physical side.
        // Our mock's side_of_pier() is computed from hour angle, so a flip happens
        // naturally as the object crosses the meridian. We just accept the command.
        // ConformU will verify by waiting for the meridian crossing.
        Ok(())
    }

    // --- Site location ---

    fn site_elevation(&self) -> AlpacaResult<f64> {
        Ok(*self.site_elevation.lock().unwrap())
    }

    fn set_site_elevation(&self, elevation: f64) -> AlpacaResult<()> {
        if !(-300.0..=10000.0).contains(&elevation) {
            return Err(AlpacaError::InvalidValue(format!(
                "Elevation {elevation} out of range (-300 to 10000 meters)"
            )));
        }
        *self.site_elevation.lock().unwrap() = elevation;
        Ok(())
    }

    fn site_latitude(&self) -> AlpacaResult<f64> {
        Ok(*self.site_latitude.lock().unwrap())
    }

    fn set_site_latitude(&self, latitude: f64) -> AlpacaResult<()> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(AlpacaError::InvalidValue(format!(
                "Latitude {latitude} out of range (-90 to 90 degrees)"
            )));
        }
        *self.site_latitude.lock().unwrap() = latitude;
        Ok(())
    }

    fn site_longitude(&self) -> AlpacaResult<f64> {
        Ok(*self.site_longitude.lock().unwrap())
    }

    fn set_site_longitude(&self, longitude: f64) -> AlpacaResult<()> {
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(AlpacaError::InvalidValue(format!(
                "Longitude {longitude} out of range (-180 to 180 degrees)"
            )));
        }
        *self.site_longitude.lock().unwrap() = longitude;
        Ok(())
    }

    fn utc_date(&self) -> AlpacaResult<String> {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let days = secs / 86400;
        let day_secs = secs % 86400;
        let h = day_secs / 3600;
        let m = (day_secs % 3600) / 60;
        let s = day_secs % 60;
        // Epoch-days-to-date (Howard Hinnant algorithm)
        let z = days + 719468;
        let era = z / 146097;
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let mo = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if mo <= 2 { y + 1 } else { y };
        Ok(format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z"))
    }

    fn set_utc_date(&self, _utc_date: &str) -> AlpacaResult<()> {
        Ok(())
    }

    // --- Axis rates ---

    fn axis_rates(&self, _axis: i32) -> AlpacaResult<Vec<AxisRates>> {
        Ok(vec![AxisRates {
            minimum: 0.0,
            maximum: 5.0,
        }])
    }

    fn can_move_axis(&self, axis: i32) -> AlpacaResult<bool> {
        // RA (0) and Dec (1) can move; tertiary axis (2) cannot
        Ok(axis == 0 || axis == 1)
    }

    // --- Sync ---

    fn sync_to_coordinates(&self, ra: f64, dec: f64) -> AlpacaResult<()> {
        self.check_not_parked()?;
        if !(0.0..24.0).contains(&ra) {
            return Err(AlpacaError::InvalidValue(format!(
                "RightAscension {ra} out of range (0 <= RA < 24)"
            )));
        }
        if !(-90.0..=90.0).contains(&dec) {
            return Err(AlpacaError::InvalidValue(format!(
                "Declination {dec} out of range (-90 <= Dec <= 90)"
            )));
        }
        *self.ra.lock().unwrap() = ra;
        *self.dec.lock().unwrap() = dec;
        *self.rate_base_ra.lock().unwrap() = ra;
        *self.rate_base_dec.lock().unwrap() = dec;
        *self.target_ra.lock().unwrap() = Some(ra);
        *self.target_dec.lock().unwrap() = Some(dec);
        Ok(())
    }

    fn sync_to_alt_az(&self, azimuth: f64, altitude: f64) -> AlpacaResult<()> {
        self.check_not_parked()?;
        if !(0.0..=360.0).contains(&azimuth) {
            return Err(AlpacaError::InvalidValue(format!(
                "Azimuth {azimuth} out of range (0 to 360 degrees)"
            )));
        }
        if !(0.0..=90.0).contains(&altitude) {
            return Err(AlpacaError::InvalidValue(format!(
                "Altitude {altitude} out of range (0 to 90 degrees)"
            )));
        }
        *self.azimuth.lock().unwrap() = azimuth;
        *self.altitude.lock().unwrap() = altitude;
        *self.target_ra.lock().unwrap() = Some(*self.ra.lock().unwrap());
        *self.target_dec.lock().unwrap() = Some(*self.dec.lock().unwrap());
        Ok(())
    }

    fn sync_to_target(&self) -> AlpacaResult<()> {
        self.check_not_parked()?;
        let ra = self.target_ra.lock().unwrap().ok_or_else(|| {
            AlpacaError::ValueNotSet("TargetRightAscension has not been set".into())
        })?;
        let dec = self.target_dec.lock().unwrap().ok_or_else(|| {
            AlpacaError::ValueNotSet("TargetDeclination has not been set".into())
        })?;
        *self.ra.lock().unwrap() = ra;
        *self.dec.lock().unwrap() = dec;
        *self.rate_base_ra.lock().unwrap() = ra;
        *self.rate_base_dec.lock().unwrap() = dec;
        Ok(())
    }

    // --- Capabilities (all true) ---

    fn alignment_mode(&self) -> AlpacaResult<AlignmentMode> {
        Ok(AlignmentMode::GermanPolar)
    }

    fn equatorial_system(&self) -> AlpacaResult<EquatorialSystem> {
        Ok(EquatorialSystem::J2000)
    }

    fn aperture_area(&self) -> AlpacaResult<f64> {
        Ok(0.0269)
    }

    fn aperture_diameter(&self) -> AlpacaResult<f64> {
        Ok(0.185)
    }

    fn focal_length(&self) -> AlpacaResult<f64> {
        Ok(1.0)
    }

    fn slew_settle_time(&self) -> AlpacaResult<i32> {
        Ok(*self.slew_settle_time.lock().unwrap())
    }

    fn set_slew_settle_time(&self, settle_time: i32) -> AlpacaResult<()> {
        if settle_time < 0 {
            return Err(AlpacaError::InvalidValue(format!(
                "SlewSettleTime must be >= 0, got {settle_time}"
            )));
        }
        *self.slew_settle_time.lock().unwrap() = settle_time;
        Ok(())
    }

    fn does_refraction(&self) -> AlpacaResult<bool> {
        Ok(*self.does_refraction.lock().unwrap())
    }

    fn set_does_refraction(&self, refraction: bool) -> AlpacaResult<()> {
        *self.does_refraction.lock().unwrap() = refraction;
        Ok(())
    }

    fn can_find_home(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_park(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_pulse_guide(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_set_declination_rate(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_set_guide_rates(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_set_park(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_set_pier_side(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_set_right_ascension_rate(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_set_tracking(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_slew(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_slew_async(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_slew_alt_az(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_slew_alt_az_async(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_sync(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_sync_alt_az(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_unpark(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
}
