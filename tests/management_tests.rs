use ascom_alpaca_core::management::{ConfiguredDevice, ServerDescription, ServerInfo};
use ascom_alpaca_core::types::DeviceType;

#[test]
fn server_description_serialization() {
    let desc = ServerDescription {
        server_name: "Nightwatch".into(),
        manufacturer: "Nightwatch Astro".into(),
        manufacturer_version: "0.1.0".into(),
        location: "Observatory".into(),
    };
    let json = serde_json::to_value(&desc).unwrap();

    assert_eq!(json["ServerName"], "Nightwatch");
    assert_eq!(json["Manufacturer"], "Nightwatch Astro");
    assert_eq!(json["ManufacturerVersion"], "0.1.0");
    assert_eq!(json["Location"], "Observatory");
}

#[test]
fn api_versions_response() {
    let info = ServerInfo::new(ServerDescription {
        server_name: "Test".into(),
        manufacturer: "Test".into(),
        manufacturer_version: "1.0".into(),
        location: "".into(),
    });
    let versions = info.api_versions();
    let json = serde_json::to_value(&versions).unwrap();
    assert_eq!(json["Value"], serde_json::json!([1]));
}

#[test]
fn configured_device_serialization() {
    let device = ConfiguredDevice {
        device_name: "Main Camera".into(),
        device_type: DeviceType::Camera,
        device_number: 0,
        unique_id: "cam-001".into(),
    };
    let json = serde_json::to_value(&device).unwrap();

    assert_eq!(json["DeviceName"], "Main Camera");
    assert_eq!(json["DeviceType"], "camera");
    assert_eq!(json["DeviceNumber"], 0);
    assert_eq!(json["UniqueID"], "cam-001");
}

#[test]
fn configured_device_roundtrip() {
    let device = ConfiguredDevice {
        device_name: "Dome".into(),
        device_type: DeviceType::Dome,
        device_number: 0,
        unique_id: "dome-001".into(),
    };
    let json_str = serde_json::to_string(&device).unwrap();
    let parsed: ConfiguredDevice = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed.device_name, "Dome");
    assert_eq!(parsed.device_type, DeviceType::Dome);
    assert_eq!(parsed.unique_id, "dome-001");
}
