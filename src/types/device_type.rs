use std::fmt;

use serde::{Deserialize, Serialize};

/// The 10 ASCOM Alpaca device types.
///
/// Serializes to/from the lowercase URL path names used in Alpaca endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
    Camera,
    CoverCalibrator,
    Dome,
    FilterWheel,
    Focuser,
    ObservingConditions,
    Rotator,
    SafetyMonitor,
    Switch,
    Telescope,
}

impl DeviceType {
    /// Returns the lowercase URL path name for this device type.
    pub fn as_path(&self) -> &'static str {
        match self {
            Self::Camera => "camera",
            Self::CoverCalibrator => "covercalibrator",
            Self::Dome => "dome",
            Self::FilterWheel => "filterwheel",
            Self::Focuser => "focuser",
            Self::ObservingConditions => "observingconditions",
            Self::Rotator => "rotator",
            Self::SafetyMonitor => "safetymonitor",
            Self::Switch => "switch",
            Self::Telescope => "telescope",
        }
    }

    /// Parses a device type from its lowercase URL path name.
    pub fn from_path(s: &str) -> Option<Self> {
        match s {
            "camera" => Some(Self::Camera),
            "covercalibrator" => Some(Self::CoverCalibrator),
            "dome" => Some(Self::Dome),
            "filterwheel" => Some(Self::FilterWheel),
            "focuser" => Some(Self::Focuser),
            "observingconditions" => Some(Self::ObservingConditions),
            "rotator" => Some(Self::Rotator),
            "safetymonitor" => Some(Self::SafetyMonitor),
            "switch" => Some(Self::Switch),
            "telescope" => Some(Self::Telescope),
            _ => None,
        }
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Camera => "Camera",
            Self::CoverCalibrator => "CoverCalibrator",
            Self::Dome => "Dome",
            Self::FilterWheel => "FilterWheel",
            Self::Focuser => "Focuser",
            Self::ObservingConditions => "ObservingConditions",
            Self::Rotator => "Rotator",
            Self::SafetyMonitor => "SafetyMonitor",
            Self::Switch => "Switch",
            Self::Telescope => "Telescope",
        };
        write!(f, "{name}")
    }
}

impl Serialize for DeviceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_path())
    }
}

impl<'de> Deserialize<'de> for DeviceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_path(&s).ok_or_else(|| serde::de::Error::custom(format!("unknown device type: {s}")))
    }
}
