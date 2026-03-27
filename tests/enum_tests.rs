use ascom_alpaca_core::types::DeviceType;

#[test]
fn device_type_path_roundtrip() {
    let types = [
        (DeviceType::Camera, "camera"),
        (DeviceType::CoverCalibrator, "covercalibrator"),
        (DeviceType::Dome, "dome"),
        (DeviceType::FilterWheel, "filterwheel"),
        (DeviceType::Focuser, "focuser"),
        (DeviceType::ObservingConditions, "observingconditions"),
        (DeviceType::Rotator, "rotator"),
        (DeviceType::SafetyMonitor, "safetymonitor"),
        (DeviceType::Switch, "switch"),
        (DeviceType::Telescope, "telescope"),
    ];

    for (dt, path) in types {
        assert_eq!(dt.as_path(), path, "as_path for {dt}");
        assert_eq!(DeviceType::from_path(path), Some(dt), "from_path for {path}");
    }
}

#[test]
fn device_type_serde_roundtrip() {
    for dt in [
        DeviceType::Camera,
        DeviceType::CoverCalibrator,
        DeviceType::Dome,
        DeviceType::FilterWheel,
        DeviceType::Focuser,
        DeviceType::ObservingConditions,
        DeviceType::Rotator,
        DeviceType::SafetyMonitor,
        DeviceType::Switch,
        DeviceType::Telescope,
    ] {
        let json = serde_json::to_string(&dt).unwrap();
        let parsed: DeviceType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, dt);
    }
}

#[test]
fn device_type_display() {
    assert_eq!(DeviceType::Camera.to_string(), "Camera");
    assert_eq!(DeviceType::CoverCalibrator.to_string(), "CoverCalibrator");
    assert_eq!(DeviceType::SafetyMonitor.to_string(), "SafetyMonitor");
}

// --- Domain enum tests ---

#[test]
fn camera_state_values() {
    use ascom_alpaca_core::camera::CameraState;

    let cases = [
        (CameraState::Idle, 0),
        (CameraState::Waiting, 1),
        (CameraState::Exposing, 2),
        (CameraState::Reading, 3),
        (CameraState::Download, 4),
        (CameraState::Error, 5),
    ];
    for (state, expected) in cases {
        let json = serde_json::to_value(state).unwrap();
        assert_eq!(json, expected, "CameraState::{state:?}");
        let parsed: CameraState = serde_json::from_value(json).unwrap();
        assert_eq!(parsed, state);
    }
}

#[test]
fn sensor_type_values() {
    use ascom_alpaca_core::camera::SensorType;

    let cases = [
        (SensorType::Monochrome, 0),
        (SensorType::Color, 1),
        (SensorType::RGGB, 2),
        (SensorType::CMYG, 3),
        (SensorType::CMYG2, 4),
        (SensorType::LRGB, 5),
    ];
    for (st, expected) in cases {
        let json = serde_json::to_value(st).unwrap();
        assert_eq!(json, expected, "SensorType::{st:?}");
    }
}

#[test]
fn guide_direction_values() {
    use ascom_alpaca_core::camera::GuideDirection;

    let cases = [
        (GuideDirection::North, 0),
        (GuideDirection::South, 1),
        (GuideDirection::East, 2),
        (GuideDirection::West, 3),
    ];
    for (dir, expected) in cases {
        let json = serde_json::to_value(dir).unwrap();
        assert_eq!(json, expected, "GuideDirection::{dir:?}");
    }
}

#[test]
fn calibrator_state_values() {
    use ascom_alpaca_core::cover_calibrator::CalibratorState;

    let cases = [
        (CalibratorState::NotPresent, 0),
        (CalibratorState::Off, 1),
        (CalibratorState::NotReady, 2),
        (CalibratorState::Ready, 3),
        (CalibratorState::Unknown, 4),
        (CalibratorState::Error, 5),
    ];
    for (state, expected) in cases {
        let json = serde_json::to_value(state).unwrap();
        assert_eq!(json, expected, "CalibratorState::{state:?}");
    }
}

#[test]
fn cover_state_values() {
    use ascom_alpaca_core::cover_calibrator::CoverState;

    let cases = [
        (CoverState::NotPresent, 0),
        (CoverState::Closed, 1),
        (CoverState::Moving, 2),
        (CoverState::Open, 3),
        (CoverState::Unknown, 4),
        (CoverState::Error, 5),
    ];
    for (state, expected) in cases {
        let json = serde_json::to_value(state).unwrap();
        assert_eq!(json, expected, "CoverState::{state:?}");
    }
}

#[test]
fn shutter_state_values() {
    use ascom_alpaca_core::dome::ShutterState;

    let cases = [
        (ShutterState::Open, 0),
        (ShutterState::Closed, 1),
        (ShutterState::Opening, 2),
        (ShutterState::Closing, 3),
        (ShutterState::Error, 4),
    ];
    for (state, expected) in cases {
        let json = serde_json::to_value(state).unwrap();
        assert_eq!(json, expected, "ShutterState::{state:?}");
    }
}

#[test]
fn alignment_mode_values() {
    use ascom_alpaca_core::telescope::AlignmentMode;

    let cases = [
        (AlignmentMode::AltAz, 0),
        (AlignmentMode::Polar, 1),
        (AlignmentMode::GermanPolar, 2),
    ];
    for (mode, expected) in cases {
        let json = serde_json::to_value(mode).unwrap();
        assert_eq!(json, expected, "AlignmentMode::{mode:?}");
    }
}

#[test]
fn side_of_pier_values() {
    use ascom_alpaca_core::telescope::SideOfPier;

    let cases = [
        (SideOfPier::East, 0),
        (SideOfPier::West, 1),
        (SideOfPier::Unknown, -1),
    ];
    for (side, expected) in cases {
        let json = serde_json::to_value(side).unwrap();
        assert_eq!(json, expected, "SideOfPier::{side:?}");
        let parsed: SideOfPier = serde_json::from_value(json).unwrap();
        assert_eq!(parsed, side);
    }
}

#[test]
fn drive_rate_values() {
    use ascom_alpaca_core::telescope::DriveRate;

    let cases = [
        (DriveRate::Sidereal, 0),
        (DriveRate::Lunar, 1),
        (DriveRate::Solar, 2),
        (DriveRate::King, 3),
    ];
    for (rate, expected) in cases {
        let json = serde_json::to_value(rate).unwrap();
        assert_eq!(json, expected, "DriveRate::{rate:?}");
    }
}

#[test]
fn equatorial_system_values() {
    use ascom_alpaca_core::telescope::EquatorialSystem;

    let cases = [
        (EquatorialSystem::Other, 0),
        (EquatorialSystem::Topocentric, 1),
        (EquatorialSystem::J2000, 2),
        (EquatorialSystem::J2050, 3),
        (EquatorialSystem::B1950, 4),
    ];
    for (sys, expected) in cases {
        let json = serde_json::to_value(sys).unwrap();
        assert_eq!(json, expected, "EquatorialSystem::{sys:?}");
    }
}

#[test]
fn axis_rates_roundtrip() {
    use ascom_alpaca_core::telescope::AxisRates;

    let rates = AxisRates {
        minimum: 0.0,
        maximum: 4.178,
    };
    let json = serde_json::to_value(&rates).unwrap();
    assert_eq!(json["Minimum"], 0.0);
    assert_eq!(json["Maximum"], 4.178);

    let parsed: AxisRates = serde_json::from_value(json).unwrap();
    assert!((parsed.minimum - 0.0).abs() < f64::EPSILON);
    assert!((parsed.maximum - 4.178).abs() < f64::EPSILON);
}

#[test]
fn typed_response_with_domain_enum() {
    use ascom_alpaca_core::camera::CameraState;
    use ascom_alpaca_core::types::AlpacaResponse;

    let resp = AlpacaResponse::ok(CameraState::Exposing);
    let json_str = serde_json::to_string(&resp).unwrap();
    let parsed: AlpacaResponse<CameraState> = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed.value, Some(CameraState::Exposing));
}
