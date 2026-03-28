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

    // --- Typed lookup methods (feature-gated) ---

    #[cfg(feature = "safety_monitor")]
    pub fn get_safety_monitor(
        &self,
        num: u32,
    ) -> Result<&dyn crate::safety_monitor::SafetyMonitor, RegistryError> {
        let idx = self
            .device_number_for(DeviceType::SafetyMonitor, num)
            .ok_or(RegistryError::DeviceNotFound {
                device_type: DeviceType::SafetyMonitor,
                device_number: num,
            })?;
        match &self.devices[idx] {
            RegisteredDevice::SafetyMonitor(d) => Ok(d.as_ref()),
            _ => Err(RegistryError::DeviceNotFound {
                device_type: DeviceType::SafetyMonitor,
                device_number: num,
            }),
        }
    }

    #[cfg(feature = "switch")]
    pub fn get_switch(&self, num: u32) -> Result<&dyn crate::switch::Switch, RegistryError> {
        let idx = self.device_number_for(DeviceType::Switch, num).ok_or(
            RegistryError::DeviceNotFound {
                device_type: DeviceType::Switch,
                device_number: num,
            },
        )?;
        match &self.devices[idx] {
            RegisteredDevice::Switch(d) => Ok(d.as_ref()),
            _ => Err(RegistryError::DeviceNotFound {
                device_type: DeviceType::Switch,
                device_number: num,
            }),
        }
    }

    #[cfg(feature = "camera")]
    pub fn get_camera(&self, num: u32) -> Result<&dyn crate::camera::Camera, RegistryError> {
        let idx = self.device_number_for(DeviceType::Camera, num).ok_or(
            RegistryError::DeviceNotFound {
                device_type: DeviceType::Camera,
                device_number: num,
            },
        )?;
        match &self.devices[idx] {
            RegisteredDevice::Camera(d) => Ok(d.as_ref()),
            _ => Err(RegistryError::DeviceNotFound {
                device_type: DeviceType::Camera,
                device_number: num,
            }),
        }
    }

    #[cfg(feature = "cover_calibrator")]
    pub fn get_cover_calibrator(
        &self,
        num: u32,
    ) -> Result<&dyn crate::cover_calibrator::CoverCalibrator, RegistryError> {
        let idx = self
            .device_number_for(DeviceType::CoverCalibrator, num)
            .ok_or(RegistryError::DeviceNotFound {
                device_type: DeviceType::CoverCalibrator,
                device_number: num,
            })?;
        match &self.devices[idx] {
            RegisteredDevice::CoverCalibrator(d) => Ok(d.as_ref()),
            _ => Err(RegistryError::DeviceNotFound {
                device_type: DeviceType::CoverCalibrator,
                device_number: num,
            }),
        }
    }

    #[cfg(feature = "dome")]
    pub fn get_dome(&self, num: u32) -> Result<&dyn crate::dome::Dome, RegistryError> {
        let idx =
            self.device_number_for(DeviceType::Dome, num)
                .ok_or(RegistryError::DeviceNotFound {
                    device_type: DeviceType::Dome,
                    device_number: num,
                })?;
        match &self.devices[idx] {
            RegisteredDevice::Dome(d) => Ok(d.as_ref()),
            _ => Err(RegistryError::DeviceNotFound {
                device_type: DeviceType::Dome,
                device_number: num,
            }),
        }
    }

    #[cfg(feature = "filter_wheel")]
    pub fn get_filter_wheel(
        &self,
        num: u32,
    ) -> Result<&dyn crate::filter_wheel::FilterWheel, RegistryError> {
        let idx = self.device_number_for(DeviceType::FilterWheel, num).ok_or(
            RegistryError::DeviceNotFound {
                device_type: DeviceType::FilterWheel,
                device_number: num,
            },
        )?;
        match &self.devices[idx] {
            RegisteredDevice::FilterWheel(d) => Ok(d.as_ref()),
            _ => Err(RegistryError::DeviceNotFound {
                device_type: DeviceType::FilterWheel,
                device_number: num,
            }),
        }
    }

    #[cfg(feature = "focuser")]
    pub fn get_focuser(&self, num: u32) -> Result<&dyn crate::focuser::Focuser, RegistryError> {
        let idx = self.device_number_for(DeviceType::Focuser, num).ok_or(
            RegistryError::DeviceNotFound {
                device_type: DeviceType::Focuser,
                device_number: num,
            },
        )?;
        match &self.devices[idx] {
            RegisteredDevice::Focuser(d) => Ok(d.as_ref()),
            _ => Err(RegistryError::DeviceNotFound {
                device_type: DeviceType::Focuser,
                device_number: num,
            }),
        }
    }

    #[cfg(feature = "observing_conditions")]
    pub fn get_observing_conditions(
        &self,
        num: u32,
    ) -> Result<&dyn crate::observing_conditions::ObservingConditions, RegistryError> {
        let idx = self
            .device_number_for(DeviceType::ObservingConditions, num)
            .ok_or(RegistryError::DeviceNotFound {
                device_type: DeviceType::ObservingConditions,
                device_number: num,
            })?;
        match &self.devices[idx] {
            RegisteredDevice::ObservingConditions(d) => Ok(d.as_ref()),
            _ => Err(RegistryError::DeviceNotFound {
                device_type: DeviceType::ObservingConditions,
                device_number: num,
            }),
        }
    }

    #[cfg(feature = "rotator")]
    pub fn get_rotator(&self, num: u32) -> Result<&dyn crate::rotator::Rotator, RegistryError> {
        let idx = self.device_number_for(DeviceType::Rotator, num).ok_or(
            RegistryError::DeviceNotFound {
                device_type: DeviceType::Rotator,
                device_number: num,
            },
        )?;
        match &self.devices[idx] {
            RegisteredDevice::Rotator(d) => Ok(d.as_ref()),
            _ => Err(RegistryError::DeviceNotFound {
                device_type: DeviceType::Rotator,
                device_number: num,
            }),
        }
    }

    #[cfg(feature = "telescope")]
    pub fn get_telescope(
        &self,
        num: u32,
    ) -> Result<&dyn crate::telescope::Telescope, RegistryError> {
        let idx = self.device_number_for(DeviceType::Telescope, num).ok_or(
            RegistryError::DeviceNotFound {
                device_type: DeviceType::Telescope,
                device_number: num,
            },
        )?;
        match &self.devices[idx] {
            RegisteredDevice::Telescope(d) => Ok(d.as_ref()),
            _ => Err(RegistryError::DeviceNotFound {
                device_type: DeviceType::Telescope,
                device_number: num,
            }),
        }
    }
}

impl Default for DeviceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
