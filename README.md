# ascom-alpaca-core

[![Crates.io](https://img.shields.io/crates/v/ascom-alpaca-core.svg)](https://crates.io/crates/ascom-alpaca-core)
[![Documentation](https://docs.rs/ascom-alpaca-core/badge.svg)](https://docs.rs/ascom-alpaca-core)
[![CI](https://github.com/nightwatch-astro/ascom-alpaca-core/actions/workflows/ci.yml/badge.svg)](https://github.com/nightwatch-astro/ascom-alpaca-core/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](#license)

Framework-agnostic ASCOM Alpaca protocol types and traits for Rust.

Provides the complete protocol abstraction for all 10 ASCOM device types (~220 methods) without any HTTP framework, async runtime, or networking dependency. Works on ESP32 (via esp-idf) and desktop alike.

## Quick Start

```bash
cargo add ascom-alpaca-core
```

Implement a device:

```rust
use ascom_alpaca_core::prelude::*;

struct MySafetyMonitor {
    wind_speed: f64,
    mount_powered: bool,
}

impl Device for MySafetyMonitor {
    fn static_name(&self) -> &str { "Observatory Safety" }
    fn unique_id(&self) -> &str { "safety-001" }
    fn device_type(&self) -> DeviceType { DeviceType::SafetyMonitor }
    fn connected(&self) -> AlpacaResult<bool> { Ok(true) }
    fn set_connected(&self, _v: bool) -> AlpacaResult<()> { Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Observatory safety monitor".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("my-safety v1.0".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok("1.0.0".into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(3) }
    fn name(&self) -> AlpacaResult<String> { Ok("Observatory Safety".into()) }
}

impl SafetyMonitor for MySafetyMonitor {
    fn is_safe(&self) -> AlpacaResult<bool> {
        Ok(self.wind_speed < 30.0 && self.mount_powered)
    }
}
```

## Core Concepts

### Device Traits

Every ASCOM device type is a Rust trait extending `Device`. The `Device` trait provides common properties (name, description, connected state, etc.), and each device trait adds type-specific methods.

All trait methods have default implementations returning `NotImplemented`, so you only override what your hardware supports:

```rust
impl Camera for MyCamera {
    // Required: what your camera actually does
    fn camera_xsize(&self) -> AlpacaResult<i32> { Ok(4656) }
    fn camera_ysize(&self) -> AlpacaResult<i32> { Ok(3520) }
    fn start_exposure(&self, duration: f64, light: bool) -> AlpacaResult<()> {
        // your exposure logic
        Ok(())
    }

    // Optional: returns NotImplemented by default
    // fn can_pulse_guide(&self) -> AlpacaResult<bool> { ... }
    // fn pulse_guide(&self, direction: GuideDirection, duration: i32) -> AlpacaResult<()> { ... }
}
```

### Response Envelopes

All Alpaca endpoints return JSON with PascalCase field names, error codes, and transaction IDs:

```rust
// Value-returning endpoint (GET)
let response = AlpacaResponse::ok(42.5)
    .with_transaction(client_tx, server_tx);
// {"Value":42.5,"ErrorNumber":0,"ErrorMessage":"","ClientTransactionID":1,"ServerTransactionID":1}

// Error response
let response = AlpacaResponse::<bool>::from_error(
    AlpacaError::InvalidValue("Temperature out of range".into())
).with_transaction(client_tx, server_tx);
// {"ErrorNumber":1025,"ErrorMessage":"Temperature out of range","ClientTransactionID":1,"ServerTransactionID":1}

// Void endpoint (PUT)
let response = MethodResponse::ok().with_transaction(client_tx, server_tx);
// {"ErrorNumber":0,"ErrorMessage":"","ClientTransactionID":1,"ServerTransactionID":1}
```

### Device Registry

The registry stores heterogeneous devices and provides typed lookup. Device numbers are assigned automatically per type (first Camera = 0, second Camera = 1, etc.):

```rust
let mut registry = DeviceRegistry::new();

let sm: Box<dyn SafetyMonitor> = Box::new(MySafetyMonitor { /* ... */ });
let cam: Box<dyn Camera> = Box::new(MyCamera { /* ... */ });
registry.register(sm);
registry.register(cam);

// For management API
let devices = registry.configured_devices();

// Typed lookup
let cam = registry.get_camera(0)?; // &dyn Camera
let sm = registry.get_safety_monitor(0)?; // &dyn SafetyMonitor

// Generic lookup (any device type)
let dev = registry.get_device(DeviceType::Camera, 0)?; // &dyn Device
```

### Error Handling

ASCOM errors are protocol-level (HTTP 200 with error in JSON body), not HTTP errors:

```rust
use ascom_alpaca_core::types::{AlpacaError, AlpacaResult};

fn set_temperature(&self, temp: f64) -> AlpacaResult<()> {
    if temp < -273.15 {
        return Err(AlpacaError::InvalidValue("Below absolute zero".into()));
    }
    if !self.is_connected() {
        return Err(AlpacaError::NotConnected("Camera not connected".into()));
    }
    // ...
    Ok(())
}
```

| Error | Code | When to use |
|-------|------|-------------|
| `NotImplemented` | 0x400 | Method not supported by this hardware |
| `InvalidValue` | 0x401 | Parameter out of valid range |
| `ValueNotSet` | 0x402 | Required value not yet assigned (e.g., target coordinates) |
| `NotConnected` | 0x407 | Device not connected |
| `InvalidWhileParked` | 0x408 | Telescope/dome operation while parked |
| `InvalidWhileSlaved` | 0x409 | Operation while dome is slaved |
| `InvalidOperationException` | 0x40B | General invalid state |
| `ActionNotImplemented` | 0x40C | Custom action not recognized |
| `OperationCancelled` | 0x40E | Async operation was cancelled |
| `DriverError` | 0x500-0xFFF | Hardware-specific errors |

### Discovery & Management

```rust
use ascom_alpaca_core::discovery::*;

// UDP discovery constants
let port = DEFAULT_DISCOVERY_PORT; // 32227
let probe = DISCOVERY_PROBE; // "alpacadiscovery1"
let ipv6 = IPV6_MULTICAST; // ff12::a1:9aca

// Management API types
use ascom_alpaca_core::management::*;

let description = ServerDescription {
    server_name: "My Alpaca Server".into(),
    manufacturer: "My Company".into(),
    manufacturer_version: "1.0.0".into(),
    location: "Backyard Observatory".into(),
};
```

### Device State

The `DeviceStateBuilder` provides an ergonomic way to construct the `device_state()` response (Platform 7+):

```rust
use ascom_alpaca_core::prelude::*;

fn device_state(&self) -> AlpacaResult<Vec<DeviceStateItem>> {
    Ok(DeviceStateBuilder::new()
        .add("IsSafe", self.is_safe)
        .add("TimeSinceLastHeartbeat", self.last_heartbeat.elapsed().as_secs())
        .add("AlarmActive", self.alarm_active)
        .add("Armed", self.armed)
        .build())
}
```

The `.add()` method accepts any type that implements `Serialize` — no manual `DeviceStateItem` construction or `serde_json::json!()` needed.

### Transaction Tracking

```rust
let counter = TransactionCounter::new();
let server_tx = counter.next(); // thread-safe atomic increment

let tracker = ClientTracker::new();
tracker.record_client(client_id);
```

## Device Types

| Trait | Methods | Description |
|-------|---------|-------------|
| [`SafetyMonitor`](#safetymonitor) | 1 | Generic unsafe condition trigger (IsSafe) |
| [`Switch`](#switch) | 16 | Multi-channel on/off and analog switches |
| [`Camera`](#camera) | ~55 | Imaging with exposure, binning, gain, cooling |
| [`CoverCalibrator`](#covercalibrator) | 12 | Flat panel and dust cover control |
| [`Dome`](#dome) | 22 | Observatory dome rotation and shutter |
| [`FilterWheel`](#filterwheel) | 7 | Filter wheel position and naming |
| [`Focuser`](#focuser) | 12 | Focus motor with temperature compensation |
| [`ObservingConditions`](#observingconditions) | 15 | Weather station (13 sensors) |
| [`Rotator`](#rotator) | 12 | Camera rotator (mechanical + logical position) |
| [`Telescope`](#telescope) | ~60 | Mount control, tracking, guiding, slewing |

### SafetyMonitor

A generic trigger for unsafe conditions — not just weather. Any condition that should halt imaging operations: wind, rain, cloud cover, door open, power failure, equipment malfunction, dew heater offline, or a dead man's switch timeout. Returns a single boolean: is it safe to continue?

```rust
impl SafetyMonitor for MyDevice {
    fn is_safe(&self) -> AlpacaResult<bool> {
        // Combine any conditions that make observing unsafe
        let weather_ok = self.wind_speed < 30.0 && !self.is_raining;
        let equipment_ok = self.dew_heater_active && self.mount_powered;
        let heartbeat_ok = self.last_heartbeat.elapsed() < Duration::from_secs(60);
        Ok(weather_ok && equipment_ok && heartbeat_ok)
    }
}
```

### Switch

Multi-channel device with boolean, multi-state, and analog channels:

```rust
impl Switch for MyPowerBox {
    fn max_switch(&self) -> AlpacaResult<i32> { Ok(4) } // 4 channels
    fn can_write(&self, id: u32) -> AlpacaResult<bool> { Ok(true) }
    fn get_switch(&self, id: u32) -> AlpacaResult<bool> { /* ... */ }
    fn set_switch(&self, id: u32, state: bool) -> AlpacaResult<()> { /* ... */ }
    fn get_switch_value(&self, id: u32) -> AlpacaResult<f64> { /* ... */ }
    fn set_switch_value(&self, id: u32, value: f64) -> AlpacaResult<()> { /* ... */ }
    fn min_switch_value(&self, id: u32) -> AlpacaResult<f64> { Ok(0.0) }
    fn max_switch_value(&self, id: u32) -> AlpacaResult<f64> { Ok(1.0) }
    fn switch_step(&self, id: u32) -> AlpacaResult<f64> { Ok(1.0) }
    // ISwitchV3 async methods optional: can_async, set_async, set_async_value, etc.
}
```

### Camera

The most complex device type. Key capability groups:

- **Exposure**: `start_exposure`, `stop_exposure`, `abort_exposure`, `image_ready`, `image_array`
- **Binning**: `bin_x/y`, `max_bin_x/y`, `can_asymmetric_bin`
- **Subframe**: `start_x/y`, `num_x/y`
- **Cooling**: `cooler_on`, `set_ccd_temperature`, `cooler_power` (gated by `can_set_ccd_temperature`)
- **Gain**: Two mutually exclusive modes:
  - *Numeric*: `gain`, `gain_min`, `gain_max` — `gains()` returns NotImplemented
  - *Named*: `gains()` returns a list — `gain_min/gain_max` return NotImplemented
- **Offset**: Same two modes as gain (`offset`/`offset_min`/`offset_max` vs `offsets()`)
- **Pulse guide**: `pulse_guide`, `is_pulse_guiding` (gated by `can_pulse_guide`)
- **Image data**: `ImageData` enum with 2D/3D i32/i16 arrays, plus ImageBytes binary encoding

### Telescope

The most method-rich device (~60 methods). Key areas:

- **Coordinates**: `right_ascension`, `declination`, `altitude`, `azimuth`, `sidereal_time`
- **Slewing**: `slew_to_coordinates[_async]`, `slew_to_alt_az[_async]`, `slew_to_target[_async]`
- **Tracking**: `tracking`, `set_tracking`, `tracking_rate`, `right_ascension_rate`, `declination_rate`
- **Guiding**: `pulse_guide`, `guide_rate_right_ascension/declination`
- **German equatorial**: `side_of_pier`, `set_side_of_pier`, `destination_side_of_pier`
- **Site**: `site_latitude`, `site_longitude`, `site_elevation`

Important ASCOM semantics:
- Sidereal tracking does NOT change the reported RA — the mount compensates for Earth's rotation
- Only `RightAscensionRate` (offset from sidereal) causes RA drift
- Guide rates are in degrees per sidereal second (RA) or degrees per SI second (Dec)

### Domain Types

All ASCOM enums serialize to their integer values via `serde_repr`:

```rust
use ascom_alpaca_core::camera::{CameraState, SensorType};
use ascom_alpaca_core::telescope::{AlignmentMode, DriveRate, SideOfPier};
use ascom_alpaca_core::dome::ShutterState;
use ascom_alpaca_core::cover_calibrator::{CoverState, CalibratorState};
use ascom_alpaca_core::types::GuideDirection;

let state = CameraState::Exposing; // serializes as 2
let side = SideOfPier::East; // serializes as 0
```

## Feature Flags

All device types are enabled by default. Disable defaults to reduce binary size on embedded:

```toml
[dependencies]
ascom-alpaca-core = { default-features = false, features = ["safety_monitor", "switch"] }
```

| Feature | Device Type | Methods |
|---------|-------------|---------|
| `camera` | Camera | ~55 |
| `cover_calibrator` | CoverCalibrator | 12 |
| `dome` | Dome | 22 |
| `filter_wheel` | FilterWheel | 7 |
| `focuser` | Focuser | 12 |
| `observing_conditions` | ObservingConditions | 15 |
| `rotator` | Rotator | 12 |
| `safety_monitor` | SafetyMonitor | 1 |
| `switch` | Switch | 16 |
| `telescope` | Telescope | ~60 |
| `all-devices` | All of the above | (default) |
| `conformu` | ConformU test harness | Adds tiny_http, dispatch layer, mock devices |

## Integration Guide

### Wiring to an HTTP Server

This crate provides protocol types only — you bring your own HTTP server. Here's the pattern:

```rust
use ascom_alpaca_core::prelude::*;
use ascom_alpaca_core::types::params::normalize_params;

// 1. Parse the URL: /api/v1/{device_type}/{device_number}/{method}
// 2. Extract query params + PUT body params, normalize keys to lowercase
// 3. Look up the device in the registry
// 4. Call the trait method
// 5. Build the response envelope

fn handle_request(
    registry: &DeviceRegistry,
    device_type: DeviceType,
    device_number: u32,
    method: &str,
    params: &HashMap<String, String>,
    is_put: bool,
    server_tx: u32,
) -> String {
    let client_tx: u32 = params.get("clienttransactionid")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    match device_type {
        DeviceType::SafetyMonitor => {
            let dev = registry.get_safety_monitor(device_number).unwrap();
            match (method, is_put) {
                ("issafe", false) => {
                    match dev.is_safe() {
                        Ok(v) => serde_json::to_string(
                            &AlpacaResponse::ok(v).with_transaction(client_tx, server_tx)
                        ).unwrap(),
                        Err(e) => serde_json::to_string(
                            &AlpacaResponse::<bool>::from_error(e).with_transaction(client_tx, server_tx)
                        ).unwrap(),
                    }
                }
                _ => { /* handle other methods */ todo!() }
            }
        }
        // ... other device types
        _ => todo!()
    }
}
```

### Management API

Every Alpaca server must expose three management endpoints:

```rust
use ascom_alpaca_core::management::*;

// GET /management/apiversions → [1]
let versions = ApiVersions { value: vec![1] };

// GET /management/v1/description → server info
let desc = ServerDescription {
    server_name: "My Server".into(),
    manufacturer: "Acme".into(),
    manufacturer_version: "1.0".into(),
    location: "Home".into(),
};

// GET /management/v1/configureddevices → device list
let devices = registry.configured_devices();
```

### UDP Discovery

Alpaca clients discover servers via UDP broadcast on port 32227:

```rust
use ascom_alpaca_core::discovery::*;

// Listen for "alpacadiscovery1" on port 32227
// Respond with: {"AlpacaPort": 32888}
let response = DiscoveryResponse { alpaca_port: 32888 };
```

### ESP32 / Embedded

The crate has no `std` networking dependency. For ESP32:

```toml
[dependencies]
ascom-alpaca-core = { default-features = false, features = ["safety_monitor"] }
```

Use with `esp_http_server` (C bindings) or any ESP-IDF HTTP server. The response types serialize to JSON via serde, which you send as the HTTP response body.

## ConformU Validation

This crate is validated against [ASCOM ConformU](https://github.com/ASCOMInitiative/ConformU) (v4.2.1), the official ASCOM conformance checker by Peter Simpson. ConformU tests are run in CI on every pull request — all 11 mock devices pass with 0 errors, 0 issues.

### Running ConformU Locally

```bash
# 1. Start the test harness
cargo run --example conformu_harness --features conformu

# 2. Test a single device
conformu conformance "http://127.0.0.1:32888/api/v1/safetymonitor/0" \
  --resultsfile results.json --logfilename conformu.log

# 3. Test all devices
for dev in safetymonitor/0 camera/0 camera/1 switch/0 covercalibrator/0 \
           dome/0 filterwheel/0 focuser/0 observingconditions/0 rotator/0 telescope/0; do
  echo "--- $dev ---"
  conformu conformance "http://127.0.0.1:32888/api/v1/$dev" \
    --resultsfile "/tmp/$(echo $dev | tr '/' '-').json"
done
```

### Custom Settings

ConformU supports a JSON settings file for fine-tuning tests (timeouts, extended tests, strictness). Pass it via `--settingsfile`:

```bash
conformu conformance "http://127.0.0.1:32888/api/v1/telescope/0" \
  --settingsfile my-settings.json
```

Key settings you might want to adjust for your device:

| Setting | Default | Description |
|---------|---------|-------------|
| `TestSideOfPierRead/Write` | `false` | Enable extended SideOfPier tests (GEM mounts) |
| `DomeOpenShutter` | `false` | Test shutter open/close (requires real or mock shutter) |
| `SwitchEnableSet` | `false` | Allow switch set operations |
| `CameraExposureDuration` | `2.0` | Exposure time in seconds |
| `ProtocolStrictChecks` | `false` | Stricter HTTP status and JSON validation |
| `TelescopeExtendedMoveAxisTests` | `true` | Extended MoveAxis validation |

See [`.github/conformu-settings.json`](.github/conformu-settings.json) for the full CI configuration with optimized timeouts for mock devices.

### CI Integration

ConformU runs as a matrix job in CI — one job per device type, in parallel. The CI uses a [settings file](.github/conformu-settings.json) with:
- Strict protocol checks enabled
- Reduced timeouts (mocks respond instantly, no need for hardware delays)
- All optional tests enabled (SideOfPier, shutter, switch set)
- Performance testing enabled

Results are uploaded as artifacts (JSON + log) for each device.

### The Harness

The harness includes:
- Mock implementations for all 10 device types with configurable capabilities
- Complete URL-to-trait dispatch layer (`dispatch_request`)
- Management API handlers
- Preemptive meridian flip (6 min before meridian, like real GEM mounts with dead zones)

### Using the Dispatch Layer

The `conformu` dispatch module provides a ready-made URL router:

```rust
use ascom_alpaca_core::conformu::dispatch::{dispatch_request, parse_device_path, AlpacaRequest};

if let Some((device_type, device_number, method)) = parse_device_path(&url_path) {
    let req = AlpacaRequest {
        device_type,
        device_number,
        method,
        params,     // HashMap<String, String>
        is_put,     // true for PUT requests
    };
    let (json_body, status_code) = dispatch_request(&registry, &req, server_tx);
    // Send json_body as HTTP response
}
```

### Mock Devices for Testing

Each mock is configurable. Example with Camera feature flags:

```rust
use ascom_alpaca_core::conformu::mocks::camera::{MockCamera, CameraFeatures, GainOffsetMode};

// Minimal camera
let cam = MockCamera::new();

// Full-featured with numeric gain
let cam = MockCamera::full_featured();

// Color camera with named gain/offset
let cam = MockCamera::with_features(CameraFeatures {
    cooler: true,
    pulse_guide: true,
    gain_mode: GainOffsetMode::Named(vec!["Low".into(), "High".into()]),
    offset_mode: GainOffsetMode::Named(vec!["Normal".into(), "Low Noise".into()]),
    sensor_type: SensorType::RGGB,
    ..Default::default()
});
```

## Comparison with `ascom-alpaca`

| | `ascom-alpaca-core` | `ascom-alpaca` |
|---|---|---|
| Protocol types | Yes | Yes |
| HTTP server | No (bring your own) | Built-in (axum) |
| Async runtime | None required | tokio required |
| ESP32 compatible | Yes | No |
| Binary size impact | Minimal (~types only) | Large (pulls in tokio, hyper, axum) |
| ConformU harness | Optional feature | No |

Use `ascom-alpaca-core` when:
- You need ESP32 or embedded support
- You have your own HTTP server (esp-idf, actix, warp, etc.)
- You want minimal dependencies
- You need the dispatch/mock layer for testing

Use [`ascom-alpaca`](https://crates.io/crates/ascom-alpaca) when:
- You want batteries-included client+server on desktop
- You're already using tokio/axum

## MSRV

Rust 1.75 or later.

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>).

Unless required by applicable law or agreed to in writing, software distributed under this license is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND.
