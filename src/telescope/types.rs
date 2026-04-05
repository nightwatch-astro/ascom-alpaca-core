use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Telescope alignment mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AlignmentMode {
    /// Altitude-azimuth (alt-az) mount.
    AltAz = 0,
    /// Polar (equatorial) mount.
    Polar = 1,
    /// German equatorial mount (GEM).
    GermanPolar = 2,
}

/// Which side of the pier the telescope is on.
///
/// Note: `Unknown = -1` requires special handling since it's a negative discriminant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideOfPier {
    /// Normal position: scope on east side, counterweight down.
    East = 0,
    /// Through-the-pole: scope on west side.
    West = 1,
    /// Side of pier is unknown or not applicable.
    Unknown = -1,
}

impl Serialize for SideOfPier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(*self as i32)
    }
}

impl<'de> Deserialize<'de> for SideOfPier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        match value {
            0 => Ok(Self::East),
            1 => Ok(Self::West),
            -1 => Ok(Self::Unknown),
            _ => Err(serde::de::Error::custom(format!(
                "unknown SideOfPier value: {value}"
            ))),
        }
    }
}

/// Telescope drive rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DriveRate {
    /// Sidereal tracking rate.
    Sidereal = 0,
    /// Lunar tracking rate.
    Lunar = 1,
    /// Solar tracking rate.
    Solar = 2,
    /// King tracking rate (corrected sidereal).
    King = 3,
}

/// Equatorial coordinate system type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum EquatorialSystem {
    /// Other/custom coordinate system.
    Other = 0,
    /// Topocentric (local apparent) coordinates.
    Topocentric = 1,
    /// J2000 equatorial coordinates.
    J2000 = 2,
    /// J2050 equatorial coordinates.
    J2050 = 3,
    /// B1950 (FK4) equatorial coordinates.
    B1950 = 4,
}

/// Axis rate range (min/max tracking rate in degrees/second).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AxisRates {
    /// Minimum rate in degrees per second.
    pub minimum: f64,
    /// Maximum rate in degrees per second.
    pub maximum: f64,
}
