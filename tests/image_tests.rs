use ascom_alpaca_core::camera::image::{imagebytes, ImageData};

#[test]
fn image_data_i32_2d_json_roundtrip() {
    let data = ImageData::I32_2D(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    let resp = data.to_response();

    assert_eq!(resp.image_type, 2);
    assert_eq!(resp.rank, 2);

    let json_str = serde_json::to_string(&resp).unwrap();
    let parsed: ascom_alpaca_core::camera::image::ImageArrayResponse =
        serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed.image_type, 2);
    assert_eq!(parsed.rank, 2);
    assert_eq!(parsed.value, serde_json::json!([[1, 2, 3], [4, 5, 6]]));
}

#[test]
fn image_data_i16_2d_json() {
    let data = ImageData::I16_2D(vec![vec![100, 200], vec![300, 400]]);
    assert_eq!(data.image_type(), 1);
    assert_eq!(data.rank(), 2);

    let resp = data.to_response();
    assert_eq!(resp.value, serde_json::json!([[100, 200], [300, 400]]));
}

#[test]
fn image_data_f64_2d_json() {
    let data = ImageData::F64_2D(vec![vec![1.5, 2.5], vec![3.5, 4.5]]);
    assert_eq!(data.image_type(), 3);
    assert_eq!(data.rank(), 2);
}

#[test]
fn image_data_i32_3d_json() {
    let data = ImageData::I32_3D(vec![
        vec![vec![1, 2], vec![3, 4]],
        vec![vec![5, 6], vec![7, 8]],
        vec![vec![9, 10], vec![11, 12]],
    ]);
    assert_eq!(data.image_type(), 2);
    assert_eq!(data.rank(), 3);
    assert_eq!(data.dimensions(), (3, 2, 2));
}

#[test]
fn imagebytes_i32_2d_roundtrip() {
    let original = ImageData::I32_2D(vec![vec![10, 20, 30], vec![40, 50, 60]]);
    let encoded = imagebytes::encode(&original, 0, 1, 42);
    let decoded = imagebytes::decode(&encoded).unwrap();

    if let ImageData::I32_2D(data) = decoded {
        assert_eq!(data, vec![vec![10, 20, 30], vec![40, 50, 60]]);
    } else {
        panic!("Expected I32_2D");
    }
}

#[test]
fn imagebytes_i16_2d_roundtrip() {
    let original = ImageData::I16_2D(vec![vec![1000, 2000], vec![3000, 4000]]);
    let encoded = imagebytes::encode(&original, 0, 0, 0);
    let decoded = imagebytes::decode(&encoded).unwrap();

    if let ImageData::I16_2D(data) = decoded {
        assert_eq!(data, vec![vec![1000, 2000], vec![3000, 4000]]);
    } else {
        panic!("Expected I16_2D");
    }
}

#[test]
fn imagebytes_f64_2d_roundtrip() {
    let original = ImageData::F64_2D(vec![vec![1.1, 2.2], vec![3.3, 4.4]]);
    let encoded = imagebytes::encode(&original, 0, 0, 0);
    let decoded = imagebytes::decode(&encoded).unwrap();

    if let ImageData::F64_2D(data) = decoded {
        assert!((data[0][0] - 1.1).abs() < f64::EPSILON);
        assert!((data[1][1] - 4.4).abs() < f64::EPSILON);
    } else {
        panic!("Expected F64_2D");
    }
}

#[test]
fn imagebytes_i32_3d_roundtrip() {
    let original = ImageData::I32_3D(vec![
        vec![vec![1, 2], vec![3, 4]],
        vec![vec![5, 6], vec![7, 8]],
        vec![vec![9, 10], vec![11, 12]],
    ]);
    let encoded = imagebytes::encode(&original, 0, 0, 0);
    let decoded = imagebytes::decode(&encoded).unwrap();

    if let ImageData::I32_3D(data) = decoded {
        assert_eq!(data[0][0][0], 1);
        assert_eq!(data[2][1][1], 12);
    } else {
        panic!("Expected I32_3D");
    }
}

#[test]
fn imagebytes_header_contains_transaction_ids() {
    let data = ImageData::I32_2D(vec![vec![1]]);
    let encoded = imagebytes::encode(&data, 0, 99, 42);

    // client_transaction_id at offset 8-12
    let client_tx = u32::from_le_bytes(encoded[8..12].try_into().unwrap());
    assert_eq!(client_tx, 99);

    // server_transaction_id at offset 12-16
    let server_tx = u32::from_le_bytes(encoded[12..16].try_into().unwrap());
    assert_eq!(server_tx, 42);
}

#[test]
fn imagebytes_large_image() {
    let rows = 768;
    let cols = 1024;
    let mut data = vec![vec![0i32; cols]; rows];
    for (i, row) in data.iter_mut().enumerate() {
        for (j, pixel) in row.iter_mut().enumerate() {
            *pixel = (i * cols + j) as i32;
        }
    }
    let original = ImageData::I32_2D(data);
    let encoded = imagebytes::encode(&original, 0, 0, 0);
    let decoded = imagebytes::decode(&encoded).unwrap();

    if let ImageData::I32_2D(result) = decoded {
        assert_eq!(result.len(), rows);
        assert_eq!(result[0].len(), cols);
        assert_eq!(result[0][0], 0);
        assert_eq!(result[767][1023], (767 * 1024 + 1023) as i32);
    } else {
        panic!("Expected I32_2D");
    }
}

#[test]
fn imagebytes_too_short_header() {
    let result = imagebytes::decode(&[0u8; 10]);
    assert!(result.is_err());
}
