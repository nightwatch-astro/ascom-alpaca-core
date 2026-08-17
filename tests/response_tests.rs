use ascom_alpaca_core::types::{AlpacaError, AlpacaResponse, MethodResponse};

#[test]
fn alpaca_response_ok_bool() {
    let resp = AlpacaResponse::ok(true).with_transaction(1, 42);
    insta::assert_json_snapshot!(resp, @r#"
    {
      "Value": true,
      "ErrorNumber": 0,
      "ErrorMessage": "",
      "ClientTransactionID": 1,
      "ServerTransactionID": 42
    }
    "#);
}

#[test]
fn alpaca_response_ok_i32() {
    let resp = AlpacaResponse::ok(42i32);
    let json = serde_json::to_value(&resp).unwrap();
    assert_eq!(json["Value"], 42);
}

#[test]
fn alpaca_response_ok_f64() {
    let resp = AlpacaResponse::ok(42.5f64);
    let json = serde_json::to_value(&resp).unwrap();
    assert!((json["Value"].as_f64().unwrap() - 42.5).abs() < f64::EPSILON);
}

#[test]
fn alpaca_response_ok_string() {
    let resp = AlpacaResponse::ok("hello".to_string());
    insta::assert_json_snapshot!(resp, @r#"
    {
      "Value": "hello",
      "ErrorNumber": 0,
      "ErrorMessage": "",
      "ClientTransactionID": 0,
      "ServerTransactionID": 0
    }
    "#);
}

#[test]
fn alpaca_response_ok_vec_string() {
    let resp = AlpacaResponse::ok(vec!["a".to_string(), "b".to_string()]);
    insta::assert_json_snapshot!(resp, @r#"
    {
      "Value": [
        "a",
        "b"
      ],
      "ErrorNumber": 0,
      "ErrorMessage": "",
      "ClientTransactionID": 0,
      "ServerTransactionID": 0
    }
    "#);
}

#[test]
fn alpaca_response_error_has_no_value() {
    let resp = AlpacaResponse::<bool>::from_error(AlpacaError::NotConnected("offline".into()));
    insta::assert_json_snapshot!(resp, @r#"
    {
      "ErrorNumber": 1031,
      "ErrorMessage": "offline",
      "ClientTransactionID": 0,
      "ServerTransactionID": 0
    }
    "#);
}

#[test]
fn alpaca_response_not_implemented() {
    let resp = AlpacaResponse::<bool>::not_implemented("pulse_guide");
    insta::assert_json_snapshot!(resp, @r#"
    {
      "ErrorNumber": 1024,
      "ErrorMessage": "pulse_guide is not implemented",
      "ClientTransactionID": 0,
      "ServerTransactionID": 0
    }
    "#);
}

#[test]
fn alpaca_response_roundtrip_bool() {
    let original = AlpacaResponse::ok(true).with_transaction(5, 100);
    let json_str = serde_json::to_string(&original).unwrap();
    let parsed: AlpacaResponse<bool> = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed.value, Some(true));
    assert_eq!(parsed.error_number, 0);
    assert_eq!(parsed.client_transaction_id, 5);
    assert_eq!(parsed.server_transaction_id, 100);
}

#[test]
fn alpaca_response_roundtrip_vec_i32() {
    let original = AlpacaResponse::ok(vec![1, 2, 3]);
    let json_str = serde_json::to_string(&original).unwrap();
    let parsed: AlpacaResponse<Vec<i32>> = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed.value, Some(vec![1, 2, 3]));
}

#[test]
fn method_response_ok() {
    let resp = MethodResponse::ok().with_transaction(1, 42);
    insta::assert_json_snapshot!(resp, @r#"
    {
      "ErrorNumber": 0,
      "ErrorMessage": "",
      "ClientTransactionID": 1,
      "ServerTransactionID": 42
    }
    "#);
}

#[test]
fn method_response_error() {
    let resp = MethodResponse::from_error(AlpacaError::InvalidWhileParked("can't slew".into()));
    insta::assert_json_snapshot!(resp, @r#"
    {
      "ErrorNumber": 1032,
      "ErrorMessage": "can't slew",
      "ClientTransactionID": 0,
      "ServerTransactionID": 0
    }
    "#);
}

#[test]
fn method_response_roundtrip() {
    let original = MethodResponse::ok().with_transaction(3, 77);
    let json_str = serde_json::to_string(&original).unwrap();
    let parsed: MethodResponse = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed.error_number, 0);
    assert_eq!(parsed.client_transaction_id, 3);
    assert_eq!(parsed.server_transaction_id, 77);
}

#[test]
fn pascal_case_field_names() {
    let resp = AlpacaResponse::ok(true);
    let json_str = serde_json::to_string(&resp).unwrap();

    assert!(json_str.contains("\"Value\""));
    assert!(json_str.contains("\"ErrorNumber\""));
    assert!(json_str.contains("\"ErrorMessage\""));
    assert!(json_str.contains("\"ClientTransactionID\""));
    assert!(json_str.contains("\"ServerTransactionID\""));
    // Verify it's not "ClientTransactionId" (wrong casing)
    assert!(!json_str.contains("TransactionId\""));
}

#[test]
fn forward_compat_extra_fields_ignored() {
    let json = r#"{
        "Value": true,
        "ErrorNumber": 0,
        "ErrorMessage": "",
        "ClientTransactionID": 0,
        "ServerTransactionID": 0,
        "FutureField": "ignored"
    }"#;
    let parsed: AlpacaResponse<bool> = serde_json::from_str(json).unwrap();
    assert_eq!(parsed.value, Some(true));
}

#[test]
fn normalize_params_lowercases_keys() {
    use ascom_alpaca_core::types::params::normalize_params;
    use std::collections::HashMap;

    let mut params = HashMap::new();
    params.insert("ClientID".to_string(), "42".to_string());
    params.insert("ClientTransactionID".to_string(), "1".to_string());
    params.insert("Connected".to_string(), "true".to_string());

    let normalized = normalize_params(params);
    assert!(normalized.contains_key("clientid"));
    assert!(normalized.contains_key("clienttransactionid"));
    assert!(normalized.contains_key("connected"));
    assert_eq!(normalized["clientid"], "42");
}
