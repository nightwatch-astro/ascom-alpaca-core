use ascom_alpaca_core::types::AlpacaError;

#[test]
fn error_codes_are_correct() {
    assert_eq!(AlpacaError::NotImplemented("".into()).error_code(), 0x400);
    assert_eq!(AlpacaError::InvalidValue("".into()).error_code(), 0x401);
    assert_eq!(AlpacaError::ValueNotSet("".into()).error_code(), 0x402);
    assert_eq!(AlpacaError::NotConnected("".into()).error_code(), 0x407);
    assert_eq!(
        AlpacaError::InvalidWhileParked("".into()).error_code(),
        0x408
    );
    assert_eq!(
        AlpacaError::InvalidWhileSlaved("".into()).error_code(),
        0x409
    );
    assert_eq!(
        AlpacaError::InvalidOperationException("".into()).error_code(),
        0x40B
    );
    assert_eq!(
        AlpacaError::ActionNotImplemented("".into()).error_code(),
        0x40C
    );
    assert_eq!(
        AlpacaError::OperationCancelled("".into()).error_code(),
        0x40E
    );
    assert_eq!(
        AlpacaError::DriverError {
            code: 0x500,
            message: "".into()
        }
        .error_code(),
        0x500
    );
    assert_eq!(
        AlpacaError::DriverError {
            code: 0xFFF,
            message: "".into()
        }
        .error_code(),
        0xFFF
    );
    assert_eq!(AlpacaError::Unknown(42).error_code(), 42);
}

#[test]
fn from_code_roundtrip() {
    let cases: Vec<(u32, &str)> = vec![
        (0x400, "not impl"),
        (0x401, "invalid"),
        (0x402, "not set"),
        (0x407, "disconnected"),
        (0x408, "parked"),
        (0x409, "slaved"),
        (0x40B, "invalid op"),
        (0x40C, "action"),
        (0x40E, "cancelled"),
        (0x500, "driver low"),
        (0xFFF, "driver high"),
    ];

    for (code, msg) in cases {
        let err = AlpacaError::from_code(code, msg.into());
        assert_eq!(
            err.error_code(),
            code as i32,
            "code mismatch for 0x{code:X}"
        );
        assert_eq!(err.error_message(), msg, "message mismatch for 0x{code:X}");
    }
}

#[test]
fn unknown_code_roundtrip() {
    let err = AlpacaError::from_code(9999, "whatever".into());
    assert!(matches!(err, AlpacaError::Unknown(9999)));
    assert_eq!(err.error_code(), 9999);
}

#[test]
fn display_includes_message() {
    let err = AlpacaError::NotConnected("telescope offline".into());
    let display = format!("{err}");
    assert!(display.contains("telescope offline"));
    assert!(display.contains("Not connected"));
}

#[test]
fn error_implements_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(AlpacaError::NotImplemented("test".into()));
    assert!(err.to_string().contains("test"));
}

#[test]
fn driver_error_range() {
    for code in [0x500, 0x600, 0x7FF, 0xFFF] {
        let err = AlpacaError::from_code(code, "driver".into());
        assert!(
            matches!(err, AlpacaError::DriverError { .. }),
            "0x{code:X} should be DriverError"
        );
    }
}
