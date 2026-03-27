use ascom_alpaca_core::discovery::{
    validate_probe, DiscoveryResponse, DEFAULT_DISCOVERY_PORT, DISCOVERY_PROBE, IPV6_MULTICAST,
};

#[test]
fn constants() {
    assert_eq!(DISCOVERY_PROBE, "alpacadiscovery1");
    assert_eq!(DEFAULT_DISCOVERY_PORT, 32227);
    assert_eq!(IPV6_MULTICAST, "ff12::a1:9aca");
}

#[test]
fn validate_probe_valid() {
    assert!(validate_probe(b"alpacadiscovery1"));
}

#[test]
fn validate_probe_invalid() {
    assert!(!validate_probe(b"alpacadiscovery2"));
    assert!(!validate_probe(b""));
    assert!(!validate_probe(b"alpacadiscovery"));
    assert!(!validate_probe(b"ALPACADISCOVERY1"));
}

#[test]
fn discovery_response_serialization() {
    let resp = DiscoveryResponse { alpaca_port: 32888 };
    let json = serde_json::to_value(&resp).unwrap();
    assert_eq!(json["AlpacaPort"], 32888);
}

#[test]
fn discovery_response_roundtrip() {
    let resp = DiscoveryResponse { alpaca_port: 32888 };
    let json_str = serde_json::to_string(&resp).unwrap();
    let parsed: DiscoveryResponse = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed.alpaca_port, 32888);
}
