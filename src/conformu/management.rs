use crate::management::ServerDescription;
use crate::registry::DeviceRegistry;
use crate::types::AlpacaResponse;

pub fn handle_api_versions(client_tx: u32, server_tx: u32) -> String {
    let resp = AlpacaResponse::ok(vec![1u32]).with_transaction(client_tx, server_tx);
    serde_json::to_string(&resp).unwrap()
}

pub fn handle_description(client_tx: u32, server_tx: u32) -> String {
    let desc = ServerDescription {
        server_name: "ConformU Test Harness".into(),
        manufacturer: "ascom-alpaca-core".into(),
        manufacturer_version: env!("CARGO_PKG_VERSION").into(),
        location: "localhost".into(),
    };
    let resp = AlpacaResponse::ok(desc).with_transaction(client_tx, server_tx);
    serde_json::to_string(&resp).unwrap()
}

pub fn handle_configured_devices(
    registry: &DeviceRegistry,
    client_tx: u32,
    server_tx: u32,
) -> String {
    let devices = registry.configured_devices();
    let resp = AlpacaResponse::ok(devices).with_transaction(client_tx, server_tx);
    serde_json::to_string(&resp).unwrap()
}
