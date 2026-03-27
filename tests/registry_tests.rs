#![cfg(feature = "all-devices")]

use ascom_alpaca_core::device::{Device, RegisteredDevice};
use ascom_alpaca_core::registry::{ClientTracker, DeviceRegistry, TransactionCounter};
use ascom_alpaca_core::safety_monitor::SafetyMonitor;
use ascom_alpaca_core::switch::Switch;
use ascom_alpaca_core::types::{AlpacaError, DeviceType};

struct TestSafetyMonitor {
    name: &'static str,
    id: &'static str,
}

impl Device for TestSafetyMonitor {
    fn static_name(&self) -> &str { self.name }
    fn unique_id(&self) -> &str { self.id }
    fn device_type(&self) -> DeviceType { DeviceType::SafetyMonitor }
}

impl SafetyMonitor for TestSafetyMonitor {
    fn is_safe(&self) -> ascom_alpaca_core::types::AlpacaResult<bool> {
        Ok(true)
    }
}

struct TestSwitch;

impl Device for TestSwitch {
    fn static_name(&self) -> &str { "Test Switch" }
    fn unique_id(&self) -> &str { "test-sw-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Switch }
}

impl Switch for TestSwitch {
    fn max_switch(&self) -> ascom_alpaca_core::types::AlpacaResult<i32> {
        Ok(2)
    }
}

#[test]
fn register_and_lookup() {
    let mut registry = DeviceRegistry::new();

    let sm: Box<dyn SafetyMonitor> = Box::new(TestSafetyMonitor { name: "SM1", id: "sm-001" });
    let sw: Box<dyn Switch> = Box::new(TestSwitch);

    registry.register(sm);
    registry.register(sw);

    let found_sm = registry.get_safety_monitor(0).unwrap();
    assert_eq!(found_sm.is_safe().unwrap(), true);

    let found_sw = registry.get_switch(0).unwrap();
    assert_eq!(found_sw.max_switch().unwrap(), 2);
}

#[test]
fn device_not_found() {
    let registry = DeviceRegistry::new();
    let result = registry.get_safety_monitor(0);
    assert!(result.is_err());
}

#[test]
fn device_numbers_per_type() {
    let mut registry = DeviceRegistry::new();

    let sm1: Box<dyn SafetyMonitor> = Box::new(TestSafetyMonitor { name: "SM1", id: "sm-001" });
    let sm2: Box<dyn SafetyMonitor> = Box::new(TestSafetyMonitor { name: "SM2", id: "sm-002" });
    let sw: Box<dyn Switch> = Box::new(TestSwitch);

    registry.register(sm1);
    registry.register(sw);
    registry.register(sm2);

    // SM1 is device 0, SM2 is device 1
    let found = registry.get_safety_monitor(1).unwrap();
    assert_eq!(found.is_safe().unwrap(), true);

    // Switch is device 0
    let found_sw = registry.get_switch(0).unwrap();
    assert_eq!(found_sw.max_switch().unwrap(), 2);
}

#[test]
fn configured_devices() {
    let mut registry = DeviceRegistry::new();

    let sm: Box<dyn SafetyMonitor> = Box::new(TestSafetyMonitor { name: "Weather Safe", id: "ws-001" });
    let sw: Box<dyn Switch> = Box::new(TestSwitch);

    registry.register(sm);
    registry.register(sw);

    let configured = registry.configured_devices();
    assert_eq!(configured.len(), 2);

    assert_eq!(configured[0].device_name, "Weather Safe");
    assert_eq!(configured[0].device_type, DeviceType::SafetyMonitor);
    assert_eq!(configured[0].device_number, 0);
    assert_eq!(configured[0].unique_id, "ws-001");

    assert_eq!(configured[1].device_name, "Test Switch");
    assert_eq!(configured[1].device_type, DeviceType::Switch);
    assert_eq!(configured[1].device_number, 0);
}

#[test]
fn configured_devices_json() {
    let mut registry = DeviceRegistry::new();
    let sm: Box<dyn SafetyMonitor> = Box::new(TestSafetyMonitor { name: "SM", id: "sm-1" });
    registry.register(sm);

    let configured = registry.configured_devices();
    let json = serde_json::to_value(&configured[0]).unwrap();

    assert_eq!(json["DeviceName"], "SM");
    assert_eq!(json["DeviceType"], "safetymonitor");
    assert_eq!(json["DeviceNumber"], 0);
    assert_eq!(json["UniqueID"], "sm-1");
}

#[test]
fn transaction_counter_monotonic() {
    let counter = TransactionCounter::new();

    let first = counter.next();
    let second = counter.next();
    let third = counter.next();

    assert_eq!(first, 1);
    assert_eq!(second, 2);
    assert_eq!(third, 3);
}

#[test]
fn transaction_counter_thread_safe() {
    use std::sync::Arc;

    let counter = Arc::new(TransactionCounter::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            (0..100).map(|_| c.next()).collect::<Vec<_>>()
        }));
    }

    let mut all_ids: Vec<u32> = handles
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect();
    all_ids.sort();
    all_ids.dedup();

    assert_eq!(all_ids.len(), 1000, "All IDs should be unique");
}

#[test]
fn client_tracker_record_and_check() {
    let mut tracker = ClientTracker::new();

    tracker.record_activity(1, 100);
    tracker.record_activity(2, 100);
    tracker.record_activity(3, 200);

    assert!(tracker.is_connected(1));
    assert!(tracker.is_connected(2));
    assert!(tracker.is_connected(3));

    // Check timeouts at t=250 with timeout=100
    // Clients 1 and 2 (last seen at 100) should time out, client 3 (at 200) should not
    let timed_out = tracker.check_timeouts(250, 100);
    assert_eq!(timed_out.len(), 2);
    assert!(!tracker.is_connected(1));
    assert!(!tracker.is_connected(2));
    assert!(tracker.is_connected(3));
}

#[test]
fn client_tracker_disconnect() {
    let mut tracker = ClientTracker::new();
    tracker.record_activity(42, 100);
    assert!(tracker.is_connected(42));

    tracker.disconnect(42);
    assert!(!tracker.is_connected(42));
}
