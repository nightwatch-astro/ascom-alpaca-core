# ascom-alpaca-core

Framework-agnostic ASCOM Alpaca protocol types and traits for Rust.

Provides the complete protocol abstraction for all 10 ASCOM device types (~220 methods) without any HTTP framework, async runtime, or networking dependency. Works on ESP32 and desktop.

## Features

- All 10 ASCOM Alpaca device type traits: Camera, CoverCalibrator, Dome, FilterWheel, Focuser, ObservingConditions, Rotator, SafetyMonitor, Switch, Telescope
- Typed response envelopes (`AlpacaResponse<T>`, `MethodResponse`) with correct PascalCase JSON serialization
- All ASCOM error codes (0x400-0xFFF) with `std::error::Error` implementation
- Domain enums with integer serialization via `serde_repr` (CameraState, ShutterState, SideOfPier, etc.)
- Device registry with typed lookup, transaction counter, and client tracking
- Discovery protocol types and constants
- Management API types (ServerDescription, ConfiguredDevice, ApiVersions)
- Camera ImageArray with JSON and ImageBytes binary encoding
- Per-device feature flags for minimal binary size

## Usage

```rust
use ascom_alpaca_core::prelude::*;

struct MySafetyMonitor { /* your hardware */ }

impl Device for MySafetyMonitor {
    fn static_name(&self) -> &str { "My Weather Station" }
    fn unique_id(&self) -> &str { "my-ws-001" }
    fn device_type(&self) -> DeviceType { DeviceType::SafetyMonitor }
}

impl SafetyMonitor for MySafetyMonitor {
    fn is_safe(&self) -> AlpacaResult<bool> {
        Ok(true) // your safety logic here
    }
}
```

Build a response:
```rust
let response = AlpacaResponse::ok(true)
    .with_transaction(client_tx_id, server_tx_id);
let json = serde_json::to_string(&response)?;
// {"Value":true,"ErrorNumber":0,"ErrorMessage":"","ClientTransactionID":1,"ServerTransactionID":42}
```

Register devices:
```rust
let mut registry = DeviceRegistry::new();
let sm: Box<dyn SafetyMonitor> = Box::new(MySafetyMonitor { /* ... */ });
registry.register(sm);

let devices = registry.configured_devices(); // for /management/v1/configureddevices
```

## Feature Flags

All device types are enabled by default via the `all-devices` feature.
Disable defaults and pick only what you need:

```toml
[dependencies]
ascom-alpaca-core = { version = "0.1", default-features = false, features = ["safety_monitor", "switch"] }
```

| Feature | Device Type |
|---------|-------------|
| `camera` | Camera (~55 methods) |
| `cover_calibrator` | CoverCalibrator (12 methods) |
| `dome` | Dome (22 methods) |
| `filter_wheel` | FilterWheel (7 methods) |
| `focuser` | Focuser (12 methods) |
| `observing_conditions` | ObservingConditions (15 methods) |
| `rotator` | Rotator (12 methods) |
| `safety_monitor` | SafetyMonitor (1 method) |
| `switch` | Switch (12 methods) |
| `telescope` | Telescope (~60 methods) |
| `all-devices` | All of the above (default) |

## Comparison with `ascom-alpaca`

The [`ascom-alpaca`](https://crates.io/crates/ascom-alpaca) crate provides a full Alpaca client+server built on axum+tokio.
It cannot compile on ESP32 or other embedded targets.

`ascom-alpaca-core` provides only the protocol layer — types, traits, and serialization — with no transport dependency.
Use it with any HTTP framework (esp-idf, axum, actix, warp) on any target.

## MSRV

Rust 1.75 or later.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
