use std::sync::Mutex;
use std::time::Instant;

use crate::device::Device;
use crate::dome::{Dome, ShutterState};
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

pub struct MockDome {
    connected: Mutex<bool>,
    shutter: Mutex<ShutterState>,
    at_home: Mutex<bool>,
    at_park: Mutex<bool>,
    azimuth: Mutex<f64>,
    altitude: Mutex<f64>,
    target_azimuth: Mutex<f64>,
    target_altitude: Mutex<f64>,
    slew_start: Mutex<Option<Instant>>,
    alt_slew_start: Mutex<Option<Instant>>,
    slaved: Mutex<bool>,
    park_azimuth: Mutex<f64>,
}

impl Default for MockDome {
    fn default() -> Self {
        Self::new()
    }
}

impl MockDome {
    pub fn new() -> Self {
        Self {
            connected: Mutex::new(false),
            shutter: Mutex::new(ShutterState::Closed),
            at_home: Mutex::new(true),
            at_park: Mutex::new(false),
            azimuth: Mutex::new(0.0),
            altitude: Mutex::new(90.0),
            target_azimuth: Mutex::new(0.0),
            target_altitude: Mutex::new(90.0),
            slew_start: Mutex::new(None),
            alt_slew_start: Mutex::new(None),
            slaved: Mutex::new(false),
            park_azimuth: Mutex::new(0.0),
        }
    }

    /// Check if azimuth/altitude slews have completed.
    fn check_slew_complete(&self) {
        let start = *self.slew_start.lock().unwrap();
        if let Some(started_at) = start {
            if started_at.elapsed().as_millis() >= 4000 {
                *self.azimuth.lock().unwrap() = *self.target_azimuth.lock().unwrap();
                *self.slew_start.lock().unwrap() = None;
            }
        }
        let alt_start = *self.alt_slew_start.lock().unwrap();
        if let Some(started_at) = alt_start {
            if started_at.elapsed().as_millis() >= 4000 {
                *self.altitude.lock().unwrap() = *self.target_altitude.lock().unwrap();
                *self.alt_slew_start.lock().unwrap() = None;
            }
        }
    }
}

impl Device for MockDome {
    fn static_name(&self) -> &str {
        "Mock Dome"
    }
    fn unique_id(&self) -> &str {
        "mock-dome-001"
    }
    fn device_type(&self) -> DeviceType {
        DeviceType::Dome
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
        Ok("Mock Dome with full azimuth/shutter control".into())
    }
    fn driver_info(&self) -> AlpacaResult<String> {
        Ok("ascom-alpaca-core mock".into())
    }
    fn driver_version(&self) -> AlpacaResult<String> {
        Ok(env!("CARGO_PKG_VERSION").into())
    }
    fn interface_version(&self) -> AlpacaResult<i32> {
        Ok(3)
    }
    fn name(&self) -> AlpacaResult<String> {
        Ok("Mock Dome".into())
    }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> {
        Ok(vec![])
    }
    fn device_state(&self) -> AlpacaResult<Vec<crate::device::common::DeviceStateItem>> {
        use crate::device::common::DeviceStateItem;
        self.check_slew_complete();
        Ok(vec![
            DeviceStateItem {
                name: "Altitude".into(),
                value: serde_json::json!(*self.altitude.lock().unwrap()),
            },
            DeviceStateItem {
                name: "AtHome".into(),
                value: serde_json::json!(*self.at_home.lock().unwrap()),
            },
            DeviceStateItem {
                name: "AtPark".into(),
                value: serde_json::json!(*self.at_park.lock().unwrap()),
            },
            DeviceStateItem {
                name: "Azimuth".into(),
                value: serde_json::json!(*self.azimuth.lock().unwrap()),
            },
            DeviceStateItem {
                name: "ShutterStatus".into(),
                value: serde_json::json!(*self.shutter.lock().unwrap() as i32),
            },
            DeviceStateItem {
                name: "Slewing".into(),
                value: serde_json::json!(
                    self.slew_start.lock().unwrap().is_some()
                        || self.alt_slew_start.lock().unwrap().is_some()
                ),
            },
        ])
    }
}

impl Dome for MockDome {
    fn altitude(&self) -> AlpacaResult<f64> {
        Ok(*self.altitude.lock().unwrap())
    }

    fn azimuth(&self) -> AlpacaResult<f64> {
        self.check_slew_complete();
        Ok(*self.azimuth.lock().unwrap())
    }

    fn at_home(&self) -> AlpacaResult<bool> {
        Ok(*self.at_home.lock().unwrap())
    }
    fn at_park(&self) -> AlpacaResult<bool> {
        Ok(*self.at_park.lock().unwrap())
    }
    fn shutter_status(&self) -> AlpacaResult<ShutterState> {
        Ok(*self.shutter.lock().unwrap())
    }

    fn slaved(&self) -> AlpacaResult<bool> {
        Ok(*self.slaved.lock().unwrap())
    }
    fn set_slaved(&self, slaved: bool) -> AlpacaResult<()> {
        *self.slaved.lock().unwrap() = slaved;
        Ok(())
    }

    fn slewing(&self) -> AlpacaResult<bool> {
        self.check_slew_complete();
        Ok(self.slew_start.lock().unwrap().is_some()
            || self.alt_slew_start.lock().unwrap().is_some())
    }

    // All capabilities enabled for comprehensive ConformU testing
    fn can_find_home(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_park(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_set_altitude(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_set_azimuth(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_set_park(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_set_shutter(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_slave(&self) -> AlpacaResult<bool> {
        Ok(true)
    }
    fn can_sync_azimuth(&self) -> AlpacaResult<bool> {
        Ok(true)
    }

    fn slew_to_azimuth(&self, azimuth: f64) -> AlpacaResult<()> {
        if !(0.0..360.0).contains(&azimuth) {
            return Err(AlpacaError::InvalidValue(format!(
                "Azimuth {azimuth} out of range 0-360"
            )));
        }
        *self.at_home.lock().unwrap() = false;
        *self.at_park.lock().unwrap() = false;
        *self.target_azimuth.lock().unwrap() = azimuth;
        *self.slew_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn slew_to_altitude(&self, altitude: f64) -> AlpacaResult<()> {
        if !(0.0..=90.0).contains(&altitude) {
            return Err(AlpacaError::InvalidValue(format!(
                "Altitude {altitude} out of range 0-90"
            )));
        }
        *self.target_altitude.lock().unwrap() = altitude;
        *self.alt_slew_start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn sync_to_azimuth(&self, azimuth: f64) -> AlpacaResult<()> {
        if !(0.0..360.0).contains(&azimuth) {
            return Err(AlpacaError::InvalidValue(format!(
                "Azimuth {azimuth} out of range 0-360"
            )));
        }
        *self.azimuth.lock().unwrap() = azimuth;
        *self.target_azimuth.lock().unwrap() = azimuth;
        *self.slew_start.lock().unwrap() = None;
        Ok(())
    }

    fn open_shutter(&self) -> AlpacaResult<()> {
        *self.shutter.lock().unwrap() = ShutterState::Open;
        Ok(())
    }

    fn close_shutter(&self) -> AlpacaResult<()> {
        *self.shutter.lock().unwrap() = ShutterState::Closed;
        Ok(())
    }

    fn park(&self) -> AlpacaResult<()> {
        let park_az = *self.park_azimuth.lock().unwrap();
        *self.azimuth.lock().unwrap() = park_az;
        *self.target_azimuth.lock().unwrap() = park_az;
        *self.at_park.lock().unwrap() = true;
        *self.at_home.lock().unwrap() = false;
        *self.slew_start.lock().unwrap() = None;
        Ok(())
    }

    fn set_park(&self) -> AlpacaResult<()> {
        *self.park_azimuth.lock().unwrap() = *self.azimuth.lock().unwrap();
        Ok(())
    }

    fn find_home(&self) -> AlpacaResult<()> {
        *self.azimuth.lock().unwrap() = 0.0;
        *self.target_azimuth.lock().unwrap() = 0.0;
        *self.at_home.lock().unwrap() = true;
        *self.at_park.lock().unwrap() = false;
        *self.slew_start.lock().unwrap() = None;
        Ok(())
    }

    fn abort_slew(&self) -> AlpacaResult<()> {
        *self.slew_start.lock().unwrap() = None;
        *self.alt_slew_start.lock().unwrap() = None;
        Ok(())
    }
}
