use crate::device::Device;
use crate::observing_conditions::ObservingConditions;
use crate::types::{AlpacaResult, DeviceType};

pub struct MockObservingConditions;

impl Device for MockObservingConditions {
    fn static_name(&self) -> &str { "Mock ObservingConditions" }
    fn unique_id(&self) -> &str { "mock-oc-001" }
    fn device_type(&self) -> DeviceType { DeviceType::ObservingConditions }
    fn connected(&self) -> AlpacaResult<bool> { Ok(true) }
    fn set_connected(&self, _: bool) -> AlpacaResult<()> { Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock ObservingConditions".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(1) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock ObservingConditions".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
}

impl ObservingConditions for MockObservingConditions {
    fn temperature(&self) -> AlpacaResult<f64> { Ok(15.0) }
    fn humidity(&self) -> AlpacaResult<f64> { Ok(60.0) }
    fn pressure(&self) -> AlpacaResult<f64> { Ok(1013.25) }
    fn dew_point(&self) -> AlpacaResult<f64> { Ok(7.0) }
    fn wind_speed(&self) -> AlpacaResult<f64> { Ok(5.0) }
    fn wind_direction(&self) -> AlpacaResult<f64> { Ok(180.0) }
    fn average_period(&self) -> AlpacaResult<f64> { Ok(0.0) }
    fn refresh(&self) -> AlpacaResult<()> { Ok(()) }
}
