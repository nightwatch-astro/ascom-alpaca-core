mod client;
mod transaction;

pub use client::ClientTracker;
pub use transaction::TransactionCounter;

use crate::device::{Device, RegisteredDevice};
use crate::management::ConfiguredDevice;
use crate::types::{DeviceType, RegistryError};

/// Stores heterogeneous ASCOM devices and provides typed lookup.
///
/// Device numbers are assigned per type: the first Camera registered gets number 0,
/// the second Camera gets number 1, etc.
pub struct DeviceRegistry {
    devices: Vec<RegisteredDevice>,
}

impl DeviceRegistry {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Registers a device. Device numbers are assigned automatically per type.
    pub fn register(&mut self, device: impl Into<RegisteredDevice>) {
        self.devices.push(device.into());
    }

    /// Returns the configured devices list for the management API.
    pub fn configured_devices(&self) -> Vec<ConfiguredDevice> {
        let mut type_counts: std::collections::HashMap<&str, u32> =
            std::collections::HashMap::new();
        let mut result = Vec::new();

        for device in &self.devices {
            let d = device.as_device();
            let dt = d.device_type();
            let path = dt.as_path();
            let number = *type_counts.get(path).unwrap_or(&0);
            type_counts.insert(path, number + 1);

            result.push(ConfiguredDevice {
                device_name: d.static_name().to_string(),
                device_type: dt,
                device_number: number,
                unique_id: d.unique_id().to_string(),
            });
        }

        result
    }

    fn device_number_for(&self, device_type: DeviceType, device_number: u32) -> Option<usize> {
        let mut count = 0u32;
        for (idx, device) in self.devices.iter().enumerate() {
            if device.device_type() == device_type {
                if count == device_number {
                    return Some(idx);
                }
                count += 1;
            }
        }
        None
    }

    /// Returns a reference to the base `Device` trait for a given type and number.
    pub fn get_device(
        &self,
        device_type: DeviceType,
        device_number: u32,
    ) -> Result<&dyn Device, RegistryError> {
        self.device_number_for(device_type, device_number)
            .map(|idx| self.devices[idx].as_device())
            .ok_or(RegistryError::DeviceNotFound {
                device_type,
                device_number,
            })
    }

}

/// Generates a typed getter method on `DeviceRegistry` that looks up a device by type
/// and number, then downcasts from `RegisteredDevice` to a specific device trait.
///
/// Each invocation produces: `pub fn $fn_name(&self, num: u32) -> Result<&dyn Trait, RegistryError>`
macro_rules! typed_getter {
    ($fn_name:ident, $feature:literal, $device_type:expr, $variant:ident, $trait_path:path) => {
        #[cfg(feature = $feature)]
        pub fn $fn_name(&self, num: u32) -> Result<&dyn $trait_path, RegistryError> {
            let idx = self
                .device_number_for($device_type, num)
                .ok_or(RegistryError::DeviceNotFound {
                    device_type: $device_type,
                    device_number: num,
                })?;
            match &self.devices[idx] {
                RegisteredDevice::$variant(d) => Ok(d.as_ref()),
                _ => Err(RegistryError::DeviceNotFound {
                    device_type: $device_type,
                    device_number: num,
                }),
            }
        }
    };
}

// --- Typed lookup methods (feature-gated) ---
impl DeviceRegistry {
    typed_getter!(get_safety_monitor, "safety_monitor", DeviceType::SafetyMonitor, SafetyMonitor, crate::safety_monitor::SafetyMonitor);
    typed_getter!(get_switch, "switch", DeviceType::Switch, Switch, crate::switch::Switch);
    typed_getter!(get_camera, "camera", DeviceType::Camera, Camera, crate::camera::Camera);
    typed_getter!(get_cover_calibrator, "cover_calibrator", DeviceType::CoverCalibrator, CoverCalibrator, crate::cover_calibrator::CoverCalibrator);
    typed_getter!(get_dome, "dome", DeviceType::Dome, Dome, crate::dome::Dome);
    typed_getter!(get_filter_wheel, "filter_wheel", DeviceType::FilterWheel, FilterWheel, crate::filter_wheel::FilterWheel);
    typed_getter!(get_focuser, "focuser", DeviceType::Focuser, Focuser, crate::focuser::Focuser);
    typed_getter!(get_observing_conditions, "observing_conditions", DeviceType::ObservingConditions, ObservingConditions, crate::observing_conditions::ObservingConditions);
    typed_getter!(get_rotator, "rotator", DeviceType::Rotator, Rotator, crate::rotator::Rotator);
    typed_getter!(get_telescope, "telescope", DeviceType::Telescope, Telescope, crate::telescope::Telescope);
}

impl Default for DeviceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
