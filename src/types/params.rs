use std::collections::HashMap;

use serde::Deserialize;

/// Common parameters included in every Alpaca request.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CommonParams {
    #[serde(default)]
    pub clientid: u32,
    #[serde(default)]
    pub clienttransactionid: u32,
}

/// Lowercases all parameter keys for case-insensitive matching per the Alpaca spec.
///
/// The ASCOM Alpaca specification requires that parameter names are treated
/// case-insensitively. This function normalizes a parameter map so that
/// downstream deserialization can use lowercase field names consistently.
pub fn normalize_params(params: HashMap<String, String>) -> HashMap<String, String> {
    params
        .into_iter()
        .map(|(k, v)| (k.to_lowercase(), v))
        .collect()
}
