# Architecture

## Overview

`ascom-alpaca-core` is a protocol abstraction layer. It sits between your hardware logic and your HTTP server:

```
Your Hardware Logic
       |
  [Device Trait]     <-- this crate
       |
  [HTTP Server]      <-- you provide (esp-idf, axum, actix, etc.)
       |
  ASCOM Alpaca Client (NINA, SGP, Voyager, etc.)
```

The crate provides:
- **Traits** defining the ASCOM device interfaces
- **Types** for protocol-level serialization (responses, errors, enums)
- **Registry** for managing multiple heterogeneous devices
- **Discovery/Management** types for the Alpaca management API

It does NOT provide:
- HTTP server or routing
- Async runtime
- Network transport
- State persistence

## Module Structure

```
types/              Protocol primitives
  response.rs       AlpacaResponse<T>, MethodResponse (JSON envelopes)
  error.rs          AlpacaError enum (0x400-0xFFF), AlpacaResult<T>
  device_type.rs    DeviceType enum with URL path mapping
  params.rs         Query/form parameter normalization
  guide_direction.rs  GuideDirection enum (shared by Camera + Telescope)

device/             Base device abstraction
  mod.rs            Device trait (common properties), RegisteredDevice enum
  common.rs         DeviceStateItem, shared types

registry/           Device management
  mod.rs            DeviceRegistry (heterogeneous storage + typed lookup)
  transaction.rs    TransactionCounter (atomic server transaction IDs)
  client.rs         ClientTracker (client ID tracking)

discovery/          UDP discovery protocol
  mod.rs            DiscoveryResponse, port constants, multicast addresses

management/         Management API
  mod.rs            ServerDescription, ConfiguredDevice, ApiVersions

{device_type}/      Per-device modules (feature-gated)
  mod.rs            Device trait + default NotImplemented methods
  types.rs          Domain enums (e.g., CameraState, SensorType, DriveRate)

camera/             Camera has extra modules:
  mod.rs            Camera trait (~55 methods)
  types.rs          CameraState, SensorType, GuideDirection
  image.rs          ImageData enum, ImageBytes binary encoding

conformu/           ConformU test infrastructure (feature-gated)
  dispatch.rs       URL path -> trait method router
  management.rs     Management API JSON handlers
  mocks/            Mock implementations for all 10 device types
```

## Key Design Decisions

### Trait-per-device-type

Each ASCOM device type is a separate Rust trait extending `Device`. This enables:
- Feature-gated compilation (only include device types you need)
- Typed registry lookup (`get_camera(0)` returns `&dyn Camera`)
- Default `NotImplemented` for optional methods

### RegisteredDevice enum

The `RegisteredDevice` enum wraps `Box<dyn DeviceTrait>` for each device type. This is the only way to store heterogeneous trait objects in a single `Vec` without unsafe code:

```rust
pub enum RegisteredDevice {
    SafetyMonitor(Box<dyn SafetyMonitor>),
    Camera(Box<dyn Camera>),
    Switch(Box<dyn Switch>),
    // ...
}
```

`From<Box<dyn T>>` impls allow `registry.register(boxed_device)` to work for any device type.

### PascalCase serialization

The ASCOM Alpaca protocol uses PascalCase field names in JSON. All response types use `#[serde(rename_all = "PascalCase")]`. Fields ending in "ID" need explicit `#[serde(rename = "...ID")]` because serde's PascalCase converts to "Id".

### Integer enum serialization

ASCOM enums are integers on the wire (e.g., `CameraState::Exposing = 2`). We use `serde_repr` for this:

```rust
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum CameraState {
    Idle = 0,
    Waiting = 1,
    Exposing = 2,
    Reading = 3,
    Download = 4,
    Error = 5,
}
```

### Error model

ASCOM has two error layers:
1. **HTTP errors** (400, 500) — for malformed requests, server failures
2. **Protocol errors** (in JSON body, HTTP 200) — for ASCOM-specific conditions

`AlpacaError` represents protocol errors. `RegistryError` represents lookup failures (which map to HTTP 400).

### No-std considerations

The crate uses `std` (for `Mutex`, `HashMap`, `Vec`, `String`) but has no networking or I/O dependencies. On ESP32 with esp-idf, `std` is available. For bare-metal `no_std` targets, the crate would need significant rework (replacing `String` with heapless alternatives, etc.).

## ConformU Architecture

The `conformu` feature adds a complete test infrastructure:

```
ConformU (external tool)
    |
  HTTP requests
    |
conformu_harness.rs (tiny_http server)
    |
  dispatch.rs (URL parser + router)
    |
  DeviceRegistry (typed lookup)
    |
  Mock device (trait implementation)
    |
  AlpacaResponse/MethodResponse (JSON serialization)
    |
  HTTP response back to ConformU
```

### Dispatch layer

`dispatch_request()` maps URL paths to trait method calls. It handles:
- Path parsing: `/api/v1/{device_type}/{number}/{method}`
- Parameter extraction from query string + PUT body
- Type conversion (string params → i32, f64, bool)
- Response envelope construction with transaction IDs
- Error mapping (trait errors → JSON error responses)

### Internal macros

The ConformU module uses two internal macros to reduce boilerplate:

- **`impl_mock_device!`** (in `mocks/mod.rs`) — generates the `impl Device for MockX` block with standard connected/disconnect/driver_info plumbing. Accepts string literals for most devices or `name_field:`/`unique_id_field:` for devices with dynamic identity (Camera). The `device_state` parameter takes a closure.
- **`typed_getter!`** (in `registry/mod.rs`) — generates feature-gated `get_<device>(&self, num) -> Result<&dyn Trait, RegistryError>` methods on `DeviceRegistry`. Each invocation maps a method name, feature flag, DeviceType variant, RegisteredDevice variant, and trait path.

These are private — crate users don't interact with them.

### DeviceStateBuilder

`DeviceStateBuilder` (in `device/common.rs`, exported in prelude) is a public API for constructing `Vec<DeviceStateItem>` without manual struct construction:

```rust
DeviceStateBuilder::new()
    .add("CameraState", state as i32)
    .add("Temperature", -10.0)
    .add("ImageReady", true)
    .build()
```

Used internally by all 10 mock `device_state()` implementations and available to crate consumers for their own device implementations.

### Mock devices

Each mock implements the full device trait with configurable capabilities:
- Time-based state transitions (using `Instant::now()`) for async operations
- `Mutex` for interior mutability (traits use `&self`)
- Builder patterns for feature configuration (e.g., `CameraFeatures`)
- Two mutually exclusive gain/offset modes tested (numeric vs named)

## Wire Format Examples

### GET (value-returning)

```
GET /api/v1/safetymonitor/0/issafe?ClientID=1&ClientTransactionID=42

200 OK
{
  "Value": true,
  "ErrorNumber": 0,
  "ErrorMessage": "",
  "ClientTransactionID": 42,
  "ServerTransactionID": 1
}
```

### PUT (void method)

```
PUT /api/v1/telescope/0/tracking
Content-Type: application/x-www-form-urlencoded

Tracking=true&ClientID=1&ClientTransactionID=43

200 OK
{
  "ErrorNumber": 0,
  "ErrorMessage": "",
  "ClientTransactionID": 43,
  "ServerTransactionID": 2
}
```

### Error response

```
GET /api/v1/telescope/0/rightascension

200 OK
{
  "ErrorNumber": 1031,
  "ErrorMessage": "Not connected",
  "ClientTransactionID": 0,
  "ServerTransactionID": 3
}
```
