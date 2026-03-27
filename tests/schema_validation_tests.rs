#![cfg(feature = "all-devices")]

use std::collections::HashMap;

/// Counts unique method paths per device type from the vendored ASCOM OpenAPI spec.
/// Each path like `/camera/{device_number}/startexposure` counts as one method.
/// Paths with both GET and PUT count as two methods (getter + setter).
fn count_methods_from_yaml() -> HashMap<String, usize> {
    let yaml_str =
        std::fs::read_to_string("tests/fixtures/AlpacaDeviceAPI_v1.yaml").expect("vendored YAML");
    let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_str).expect("valid YAML");

    let paths = yaml["paths"].as_mapping().expect("paths object");
    let mut counts: HashMap<String, usize> = HashMap::new();

    for (path_key, path_value) in paths {
        let path = path_key.as_str().unwrap_or("");

        // Extract device type from path: /{device_type}/{device_number}/method
        // or /camera/{device_number}/method etc.
        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        if segments.len() < 3 {
            continue;
        }

        let device_type = segments[0];

        // Skip generic {device_type} paths — those are common methods
        if device_type == "{device_type}" {
            // Count common methods separately
            let method_count = path_value.as_mapping().map_or(0, |m| {
                m.keys()
                    .filter(|k| {
                        let s = k.as_str().unwrap_or("");
                        s == "get" || s == "put"
                    })
                    .count()
            });
            *counts.entry("common".to_string()).or_insert(0) += method_count;
            continue;
        }

        // Count HTTP methods (get and put) for device-specific paths
        let method_count = path_value.as_mapping().map_or(0, |m| {
            m.keys()
                .filter(|k| {
                    let s = k.as_str().unwrap_or("");
                    s == "get" || s == "put"
                })
                .count()
        });

        *counts.entry(device_type.to_string()).or_insert(0) += method_count;
    }

    counts
}

#[test]
fn method_counts_match_official_spec() {
    let counts = count_methods_from_yaml();

    // These are the expected method counts per device type from the ASCOM spec.
    // Common methods (shared by all devices) are counted separately.
    // Each GET = 1 method, each PUT = 1 method (e.g., GET gain + PUT gain = 2 methods).

    // Verify we have entries for all 10 device types
    let device_types = [
        "camera",
        "covercalibrator",
        "dome",
        "filterwheel",
        "focuser",
        "observingconditions",
        "rotator",
        "safetymonitor",
        "switch",
        "telescope",
    ];

    for dt in &device_types {
        assert!(
            counts.contains_key(*dt),
            "Missing device type in YAML: {dt}"
        );
    }

    // Verify common methods exist
    assert!(
        counts.contains_key("common"),
        "Missing common methods in YAML"
    );
    let common_count = counts["common"];
    assert!(
        common_count >= 15,
        "Expected at least 15 common methods, got {common_count}"
    );

    // Print actual counts for documentation
    println!("Method counts from official ASCOM OpenAPI spec:");
    println!("  Common: {common_count}");
    for dt in &device_types {
        let c = counts.get(*dt).unwrap_or(&0);
        println!("  {dt}: {c}");
    }

    // Verify SafetyMonitor is the simplest (1 device-specific method)
    assert_eq!(
        counts["safetymonitor"], 1,
        "SafetyMonitor should have exactly 1 device-specific method (issafe GET)"
    );

    // Verify Camera and Telescope are the most complex
    assert!(
        counts["camera"] > 40,
        "Camera should have 40+ device-specific methods, got {}",
        counts["camera"]
    );
    assert!(
        counts["telescope"] > 50,
        "Telescope should have 50+ device-specific methods, got {}",
        counts["telescope"]
    );
}

#[test]
fn response_schema_pascal_case_fields() {
    // Verify our response types match the field naming in the ASCOM spec
    let yaml_str =
        std::fs::read_to_string("tests/fixtures/AlpacaDeviceAPI_v1.yaml").expect("vendored YAML");

    // The ASCOM spec uses PascalCase for response fields
    assert!(yaml_str.contains("ClientTransactionID"));
    assert!(yaml_str.contains("ServerTransactionID"));
    assert!(yaml_str.contains("ErrorNumber"));
    assert!(yaml_str.contains("ErrorMessage"));

    // Verify our serialization matches
    let resp = ascom_alpaca_core::types::AlpacaResponse::ok(true);
    let json = serde_json::to_string(&resp).unwrap();
    assert!(json.contains("ClientTransactionID"));
    assert!(json.contains("ServerTransactionID"));
    assert!(json.contains("ErrorNumber"));
    assert!(json.contains("ErrorMessage"));
}

#[test]
fn management_api_structure() {
    let yaml_str = std::fs::read_to_string("tests/fixtures/AlpacaManagementAPI_v1.yaml")
        .expect("vendored management YAML");
    let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_str).expect("valid YAML");

    let paths = yaml["paths"].as_mapping().expect("paths object");

    // Verify expected management endpoints exist
    let expected_paths = [
        "/management/apiversions",
        "/management/v1/description",
        "/management/v1/configureddevices",
    ];

    for expected in &expected_paths {
        assert!(
            paths.keys().any(|k| k.as_str() == Some(expected)),
            "Missing management path: {expected}"
        );
    }
}
