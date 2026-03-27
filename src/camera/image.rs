use serde::{Deserialize, Serialize};

/// Pixel data for an image array.
#[derive(Debug, Clone)]
pub enum ImageData {
    I16_2D(Vec<Vec<i16>>),
    I32_2D(Vec<Vec<i32>>),
    F64_2D(Vec<Vec<f64>>),
    I16_3D(Vec<Vec<Vec<i16>>>),
    I32_3D(Vec<Vec<Vec<i32>>>),
    F64_3D(Vec<Vec<Vec<f64>>>),
}

/// Image array with metadata, supporting both JSON and ImageBytes encoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageArrayResponse {
    /// 1 = i16, 2 = i32, 3 = f64
    #[serde(rename = "Type")]
    pub image_type: i32,
    /// 2 = monochrome, 3 = color
    pub rank: i32,
    /// The image data as nested arrays.
    pub value: serde_json::Value,
}

impl ImageData {
    /// Returns the ASCOM image element type code.
    pub fn image_type(&self) -> i32 {
        match self {
            Self::I16_2D(_) | Self::I16_3D(_) => 1,
            Self::I32_2D(_) | Self::I32_3D(_) => 2,
            Self::F64_2D(_) | Self::F64_3D(_) => 3,
        }
    }

    /// Returns the rank (2 = monochrome, 3 = color).
    pub fn rank(&self) -> i32 {
        match self {
            Self::I16_2D(_) | Self::I32_2D(_) | Self::F64_2D(_) => 2,
            Self::I16_3D(_) | Self::I32_3D(_) | Self::F64_3D(_) => 3,
        }
    }

    /// Converts to a JSON value for the response.
    pub fn to_json_value(&self) -> serde_json::Value {
        match self {
            Self::I16_2D(data) => serde_json::to_value(data).unwrap_or_default(),
            Self::I32_2D(data) => serde_json::to_value(data).unwrap_or_default(),
            Self::F64_2D(data) => serde_json::to_value(data).unwrap_or_default(),
            Self::I16_3D(data) => serde_json::to_value(data).unwrap_or_default(),
            Self::I32_3D(data) => serde_json::to_value(data).unwrap_or_default(),
            Self::F64_3D(data) => serde_json::to_value(data).unwrap_or_default(),
        }
    }

    /// Creates an `ImageArrayResponse` from this data.
    pub fn to_response(&self) -> ImageArrayResponse {
        ImageArrayResponse {
            image_type: self.image_type(),
            rank: self.rank(),
            value: self.to_json_value(),
        }
    }

    /// Returns the dimensions of the image.
    pub fn dimensions(&self) -> (usize, usize, usize) {
        match self {
            Self::I16_2D(data) => (data.len(), data.first().map_or(0, |r| r.len()), 0),
            Self::I32_2D(data) => (data.len(), data.first().map_or(0, |r| r.len()), 0),
            Self::F64_2D(data) => (data.len(), data.first().map_or(0, |r| r.len()), 0),
            Self::I16_3D(data) => (
                data.len(),
                data.first().map_or(0, |p| p.len()),
                data.first().and_then(|p| p.first()).map_or(0, |r| r.len()),
            ),
            Self::I32_3D(data) => (
                data.len(),
                data.first().map_or(0, |p| p.len()),
                data.first().and_then(|p| p.first()).map_or(0, |r| r.len()),
            ),
            Self::F64_3D(data) => (
                data.len(),
                data.first().map_or(0, |p| p.len()),
                data.first().and_then(|p| p.first()).map_or(0, |r| r.len()),
            ),
        }
    }
}

/// ImageBytes binary format header constants.
pub mod imagebytes {
    /// Current metadata version.
    pub const METADATA_VERSION: i32 = 1;

    /// Serializes image data to the ImageBytes binary format.
    ///
    /// Format: metadata header + raw pixel data in little-endian byte order.
    pub fn encode(
        data: &super::ImageData,
        error_number: i32,
        client_transaction_id: u32,
        server_transaction_id: u32,
    ) -> Vec<u8> {
        let (dim1, dim2, dim3) = data.dimensions();
        let image_type = data.image_type();
        let rank = data.rank();

        // Calculate data start offset (header size)
        // Header: metadata_version(4) + error_number(4) + client_tx(4) + server_tx(4)
        //       + data_start(4) + image_element_type(4) + transmission_element_type(4)
        //       + rank(4) + dim1(4) + dim2(4) + dim3(4) = 44 bytes
        let data_start: i32 = 44;

        let mut buf = Vec::new();

        // Header
        buf.extend_from_slice(&METADATA_VERSION.to_le_bytes());
        buf.extend_from_slice(&error_number.to_le_bytes());
        buf.extend_from_slice(&client_transaction_id.to_le_bytes());
        buf.extend_from_slice(&server_transaction_id.to_le_bytes());
        buf.extend_from_slice(&data_start.to_le_bytes());
        buf.extend_from_slice(&image_type.to_le_bytes());
        buf.extend_from_slice(&image_type.to_le_bytes()); // transmission type = image type
        buf.extend_from_slice(&rank.to_le_bytes());
        buf.extend_from_slice(&(dim1 as i32).to_le_bytes());
        buf.extend_from_slice(&(dim2 as i32).to_le_bytes());
        buf.extend_from_slice(&(dim3 as i32).to_le_bytes());

        // Pixel data
        match data {
            super::ImageData::I16_2D(rows) => {
                for row in rows {
                    for &pixel in row {
                        buf.extend_from_slice(&pixel.to_le_bytes());
                    }
                }
            }
            super::ImageData::I32_2D(rows) => {
                for row in rows {
                    for &pixel in row {
                        buf.extend_from_slice(&pixel.to_le_bytes());
                    }
                }
            }
            super::ImageData::F64_2D(rows) => {
                for row in rows {
                    for &pixel in row {
                        buf.extend_from_slice(&pixel.to_le_bytes());
                    }
                }
            }
            super::ImageData::I16_3D(planes) => {
                for plane in planes {
                    for row in plane {
                        for &pixel in row {
                            buf.extend_from_slice(&pixel.to_le_bytes());
                        }
                    }
                }
            }
            super::ImageData::I32_3D(planes) => {
                for plane in planes {
                    for row in plane {
                        for &pixel in row {
                            buf.extend_from_slice(&pixel.to_le_bytes());
                        }
                    }
                }
            }
            super::ImageData::F64_3D(planes) => {
                for plane in planes {
                    for row in plane {
                        for &pixel in row {
                            buf.extend_from_slice(&pixel.to_le_bytes());
                        }
                    }
                }
            }
        }

        buf
    }

    /// Decodes ImageBytes binary format back into ImageData.
    pub fn decode(bytes: &[u8]) -> Result<super::ImageData, String> {
        if bytes.len() < 44 {
            return Err("ImageBytes header too short".into());
        }

        let _metadata_version = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let _error_number = i32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let _client_tx = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let _server_tx = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let data_start = i32::from_le_bytes(bytes[16..20].try_into().unwrap()) as usize;
        let image_type = i32::from_le_bytes(bytes[20..24].try_into().unwrap());
        let _transmission_type = i32::from_le_bytes(bytes[24..28].try_into().unwrap());
        let rank = i32::from_le_bytes(bytes[28..32].try_into().unwrap());
        let dim1 = i32::from_le_bytes(bytes[32..36].try_into().unwrap()) as usize;
        let dim2 = i32::from_le_bytes(bytes[36..40].try_into().unwrap()) as usize;
        let dim3 = i32::from_le_bytes(bytes[40..44].try_into().unwrap()) as usize;

        let pixel_data = &bytes[data_start..];

        match (image_type, rank) {
            (1, 2) => {
                let mut data = vec![vec![0i16; dim2]; dim1];
                for (i, row) in data.iter_mut().enumerate() {
                    for (j, pixel) in row.iter_mut().enumerate() {
                        let offset = (i * dim2 + j) * 2;
                        *pixel =
                            i16::from_le_bytes(pixel_data[offset..offset + 2].try_into().unwrap());
                    }
                }
                Ok(super::ImageData::I16_2D(data))
            }
            (2, 2) => {
                let mut data = vec![vec![0i32; dim2]; dim1];
                for (i, row) in data.iter_mut().enumerate() {
                    for (j, pixel) in row.iter_mut().enumerate() {
                        let offset = (i * dim2 + j) * 4;
                        *pixel =
                            i32::from_le_bytes(pixel_data[offset..offset + 4].try_into().unwrap());
                    }
                }
                Ok(super::ImageData::I32_2D(data))
            }
            (3, 2) => {
                let mut data = vec![vec![0.0f64; dim2]; dim1];
                for (i, row) in data.iter_mut().enumerate() {
                    for (j, pixel) in row.iter_mut().enumerate() {
                        let offset = (i * dim2 + j) * 8;
                        *pixel =
                            f64::from_le_bytes(pixel_data[offset..offset + 8].try_into().unwrap());
                    }
                }
                Ok(super::ImageData::F64_2D(data))
            }
            (1, 3) => {
                let mut data = vec![vec![vec![0i16; dim3]; dim2]; dim1];
                for (i, plane) in data.iter_mut().enumerate() {
                    for (j, row) in plane.iter_mut().enumerate() {
                        for (k, pixel) in row.iter_mut().enumerate() {
                            let offset = (i * dim2 * dim3 + j * dim3 + k) * 2;
                            *pixel = i16::from_le_bytes(
                                pixel_data[offset..offset + 2].try_into().unwrap(),
                            );
                        }
                    }
                }
                Ok(super::ImageData::I16_3D(data))
            }
            (2, 3) => {
                let mut data = vec![vec![vec![0i32; dim3]; dim2]; dim1];
                for (i, plane) in data.iter_mut().enumerate() {
                    for (j, row) in plane.iter_mut().enumerate() {
                        for (k, pixel) in row.iter_mut().enumerate() {
                            let offset = (i * dim2 * dim3 + j * dim3 + k) * 4;
                            *pixel = i32::from_le_bytes(
                                pixel_data[offset..offset + 4].try_into().unwrap(),
                            );
                        }
                    }
                }
                Ok(super::ImageData::I32_3D(data))
            }
            (3, 3) => {
                let mut data = vec![vec![vec![0.0f64; dim3]; dim2]; dim1];
                for (i, plane) in data.iter_mut().enumerate() {
                    for (j, row) in plane.iter_mut().enumerate() {
                        for (k, pixel) in row.iter_mut().enumerate() {
                            let offset = (i * dim2 * dim3 + j * dim3 + k) * 8;
                            *pixel = f64::from_le_bytes(
                                pixel_data[offset..offset + 8].try_into().unwrap(),
                            );
                        }
                    }
                }
                Ok(super::ImageData::F64_3D(data))
            }
            _ => Err(format!("unsupported image type {image_type} rank {rank}")),
        }
    }
}
