# Contributing to ascom-alpaca-core

## Getting Started

```bash
git clone https://github.com/nightwatch-astro/ascom-alpaca-core.git
cd ascom-alpaca-core
cargo build
cargo test
```

## Development

### Building

```bash
cargo build                              # default (all-devices)
cargo build --features conformu          # includes ConformU test harness
cargo build --no-default-features --features safety_monitor  # single device
```

### Testing

```bash
cargo test                               # unit + schema validation tests
cargo clippy                             # lint
cargo fmt --check                        # format check
```

### ConformU Conformance Testing

The `conformu` feature includes mock devices and a test harness for validating against [ASCOM ConformU](https://github.com/ASCOMInitiative/ConformU):

```bash
# Start the test harness
cargo run --example conformu_harness --features conformu

# In another terminal, run ConformU against a device
conformu conformance "http://127.0.0.1:32888/api/v1/safetymonitor/0" \
  --resultsfile results.json --logfilename conformu.log

# Full sweep (all devices)
for dev in safetymonitor/0 camera/0 camera/1 switch/0 covercalibrator/0 \
           dome/0 filterwheel/0 focuser/0 observingconditions/0 rotator/0 telescope/0; do
  echo "--- $dev ---"
  conformu conformance "http://127.0.0.1:32888/api/v1/$dev" \
    --resultsfile "/tmp/$(echo $dev | tr '/' '-').json" \
    --logfilename "/tmp/$(echo $dev | tr '/' '-').log"
done
```

All 11 mock devices (including both mono and color cameras) must pass with 0 errors and 0 issues.

## Architecture

```
src/
  types/           Core protocol types (errors, responses, params, device types)
  device/          Device + RegisteredDevice traits, DeviceState
  registry/        DeviceRegistry, TransactionCounter, ClientTracker
  discovery/       UDP discovery protocol types
  management/      Management API types (ServerDescription, ConfiguredDevice)
  camera/          Camera trait + types (CameraState, SensorType, ImageData)
  telescope/       Telescope trait + types (DriveRate, SideOfPier, AxisRates)
  {device}/        One module per device type, each with trait + domain types
  conformu/        (feature-gated) Test harness, dispatch layer, mock devices
    dispatch.rs    URL-to-trait router for all 10 device types
    management.rs  Management API handlers
    mocks/         Configurable mock implementations for ConformU testing
```

### Design Principles

1. **No transport dependency** — types and traits only; works with any HTTP server
2. **Feature-gated device types** — only compile what you need for embedded targets
3. **Default NotImplemented** — trait methods default to `NotImplemented` so you only override what your hardware supports
4. **PascalCase JSON** — all serialization matches the ASCOM Alpaca wire format exactly
5. **Integer enums** — domain enums serialize to their ASCOM integer values via `serde_repr`

## Adding a New Device Type

1. Create `src/{device_type}/mod.rs` with the trait (extending `Device`)
2. Add domain types in `src/{device_type}/types.rs` if needed
3. Add a feature flag in `Cargo.toml` and wire it into `all-devices`
4. Add the variant to `RegisteredDevice` in `src/device/mod.rs` (+ `From` impl)
5. Add a `typed_getter!` invocation in `src/registry/mod.rs`
6. Re-export in `src/prelude` (feature-gated)
7. Add dispatch in `src/conformu/dispatch.rs` (feature-gated)
8. Add a mock in `src/conformu/mocks/` using `impl_mock_device!` macro + `DeviceStateBuilder`
9. Register the mock in `examples/conformu_harness.rs`

## Adding Methods to an Existing Device

1. Add the method to the trait with a default `NotImplemented` implementation
2. Add dispatch routing in `src/conformu/dispatch.rs`
3. Implement in the relevant mock
4. Run ConformU to validate

## Commit Convention

This project uses [conventional commits](https://www.conventionalcommits.org/):

```
feat(camera): add SubExposureDuration support
fix(conformu): telescope sidereal tracking drift
docs: comprehensive README with integration guide
test: schema validation for new endpoints
refactor(registry): simplify typed lookup
```

Scopes: `camera`, `telescope`, `dome`, `switch`, `focuser`, `rotator`, `conformu`, `registry`, `types`, etc.

## Code Style

- `cargo fmt` for formatting
- `cargo clippy` for linting
- Trait methods return `AlpacaResult<T>` (never panic)
- Use `AlpacaError` variants, not custom error types
- Mocks use `Mutex` for interior mutability (single-threaded harness, but trait requires `&self`)
- Mocks use `impl_mock_device!` macro for `Device` trait boilerplate (see `conformu/mocks/mod.rs`)
- Use `DeviceStateBuilder` for `device_state()` implementations (both in mocks and consumer code)

## License

By contributing, you agree that your contributions will be licensed under the Apache License, Version 2.0.
