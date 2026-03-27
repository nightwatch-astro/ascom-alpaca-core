use serde::{Deserialize, Serialize};

/// The probe string sent by Alpaca clients for device discovery.
pub const DISCOVERY_PROBE: &str = "alpacadiscovery1";

/// The default UDP port for Alpaca discovery.
pub const DEFAULT_DISCOVERY_PORT: u16 = 32227;

/// The IPv6 multicast address for Alpaca discovery.
pub const IPV6_MULTICAST: &str = "ff12::a1:9aca";

/// Response to an Alpaca discovery probe.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiscoveryResponse {
    pub alpaca_port: u16,
}

/// Validates that a byte payload is a valid Alpaca discovery probe.
pub fn validate_probe(bytes: &[u8]) -> bool {
    bytes == DISCOVERY_PROBE.as_bytes()
}
