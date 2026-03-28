#![cfg(feature = "all-devices")]

use ascom_alpaca_core::device::Device;
use ascom_alpaca_core::safety_monitor::SafetyMonitor;
use ascom_alpaca_core::switch::Switch;
use ascom_alpaca_core::types::{AlpacaError, DeviceType};

struct MockSafetyMonitor;

impl Device for MockSafetyMonitor {
    fn static_name(&self) -> &str {
        "Test Safety Monitor"
    }
    fn unique_id(&self) -> &str {
        "test-sm-001"
    }
    fn device_type(&self) -> DeviceType {
        DeviceType::SafetyMonitor
    }
}

impl SafetyMonitor for MockSafetyMonitor {
    fn is_safe(&self) -> ascom_alpaca_core::types::AlpacaResult<bool> {
        Ok(true)
    }
}

struct MockSwitch;

impl Device for MockSwitch {
    fn static_name(&self) -> &str {
        "Test Switch"
    }
    fn unique_id(&self) -> &str {
        "test-sw-001"
    }
    fn device_type(&self) -> DeviceType {
        DeviceType::Switch
    }
}

impl Switch for MockSwitch {
    fn max_switch(&self) -> ascom_alpaca_core::types::AlpacaResult<i32> {
        Ok(4)
    }

    fn get_switch(&self, id: u32) -> ascom_alpaca_core::types::AlpacaResult<bool> {
        Ok(id % 2 == 0)
    }
}

#[test]
fn safety_monitor_overridden_method() {
    let sm = MockSafetyMonitor;
    assert!(sm.is_safe().unwrap());
}

#[test]
fn safety_monitor_identity() {
    let sm = MockSafetyMonitor;
    assert_eq!(sm.static_name(), "Test Safety Monitor");
    assert_eq!(sm.unique_id(), "test-sm-001");
    assert_eq!(sm.device_type(), DeviceType::SafetyMonitor);
}

#[test]
fn device_defaults_return_not_implemented() {
    let sm = MockSafetyMonitor;

    // All Device default methods should return NotImplemented
    assert!(matches!(
        sm.connected(),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sm.set_connected(true),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sm.connecting(),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(sm.connect(), Err(AlpacaError::NotImplemented(_))));
    assert!(matches!(
        sm.disconnect(),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sm.description(),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sm.driver_info(),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sm.driver_version(),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sm.interface_version(),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(sm.name(), Err(AlpacaError::NotImplemented(_))));
    assert!(matches!(
        sm.supported_actions(),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sm.action("test", ""),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sm.command_blind("test", false),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sm.command_bool("test", false),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sm.command_string("test", false),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sm.device_state(),
        Err(AlpacaError::NotImplemented(_))
    ));
}

#[test]
fn switch_overridden_and_default_methods() {
    let sw = MockSwitch;

    assert_eq!(sw.max_switch().unwrap(), 4);
    assert!(sw.get_switch(0).unwrap());
    assert!(!sw.get_switch(1).unwrap());

    // Unoverridden Switch methods return NotImplemented
    assert!(matches!(
        sw.can_write(0),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sw.set_switch(0, true),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sw.get_switch_value(0),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sw.get_switch_name(0),
        Err(AlpacaError::NotImplemented(_))
    ));
    assert!(matches!(
        sw.get_switch_description(0),
        Err(AlpacaError::NotImplemented(_))
    ));
}

#[test]
fn device_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MockSafetyMonitor>();
    assert_send_sync::<MockSwitch>();
}

#[test]
fn trait_object_works() {
    let sm = MockSafetyMonitor;
    let device: &dyn Device = &sm;
    assert_eq!(device.static_name(), "Test Safety Monitor");
    assert_eq!(device.device_type(), DeviceType::SafetyMonitor);
}
