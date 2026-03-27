# ascom-alpaca-core development tasks

# Run all tests with all features
test:
    cargo test --all-features

# Run clippy
clippy:
    cargo clippy --all-features -- -D warnings

# Format check
fmt:
    cargo fmt --check

# Check all feature combinations
check-features:
    cargo check --all-features
    cargo check --no-default-features
    cargo check --no-default-features --features safety_monitor
    cargo check --no-default-features --features camera
    cargo check --no-default-features --features telescope

# Vendor ASCOM OpenAPI specs from ASCOMInitiative/ASCOMRemote
vendor-specs:
    curl -sL -o tests/fixtures/AlpacaDeviceAPI_v1.yaml \
        "https://raw.githubusercontent.com/ASCOMInitiative/ASCOMRemote/master/Swagger/AlpacaDeviceAPI_v1.yaml"
    curl -sL -o tests/fixtures/AlpacaManagementAPI_v1.yaml \
        "https://raw.githubusercontent.com/ASCOMInitiative/ASCOMRemote/master/Swagger/AlpacaManagementAPI_v1.yaml"
    @echo "Vendored ASCOM OpenAPI specs to tests/fixtures/"

# Dry-run publish
publish-dry-run:
    cargo publish --dry-run
