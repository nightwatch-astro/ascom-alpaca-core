/// Generates the `impl Device for $struct` block with standard mock plumbing.
///
/// All mock devices share identical `connected`/`connect`/`disconnect`/`driver_info`/
/// `driver_version`/`supported_actions` implementations. This macro generates those plus
/// the identity methods (`static_name`, `unique_id`, `device_type`, `interface_version`,
/// `name`, `description`) from parameters, and delegates `device_state` to a closure.
///
/// The `name` and `unique_id` parameters accept string literals. For dynamic values
/// (e.g., fields), use `name_field:` and `unique_id_field:` with the field identifier.
///
/// Requires the struct to have a `connected: Mutex<bool>` field.
macro_rules! impl_mock_device {
    // Variant with field references for name/unique_id (e.g., Camera)
    (
        $struct_name:ty,
        name_field: $name_field:ident,
        unique_id_field: $unique_id_field:ident,
        device_type: $device_type:expr,
        interface_version: $version:literal,
        device_state: $state_fn:expr
    ) => {
        impl crate::device::Device for $struct_name {
            fn static_name(&self) -> &str {
                &self.$name_field
            }
            fn unique_id(&self) -> &str {
                &self.$unique_id_field
            }
            fn device_type(&self) -> crate::types::DeviceType {
                $device_type
            }
            fn connected(&self) -> crate::types::AlpacaResult<bool> {
                Ok(*self.connected.lock().unwrap())
            }
            fn set_connected(&self, v: bool) -> crate::types::AlpacaResult<()> {
                *self.connected.lock().unwrap() = v;
                Ok(())
            }
            fn connecting(&self) -> crate::types::AlpacaResult<bool> {
                Ok(false)
            }
            fn connect(&self) -> crate::types::AlpacaResult<()> {
                *self.connected.lock().unwrap() = true;
                Ok(())
            }
            fn disconnect(&self) -> crate::types::AlpacaResult<()> {
                *self.connected.lock().unwrap() = false;
                Ok(())
            }
            fn description(&self) -> crate::types::AlpacaResult<String> {
                Ok(format!("{} for ConformU testing", self.$name_field))
            }
            fn driver_info(&self) -> crate::types::AlpacaResult<String> {
                Ok("ascom-alpaca-core mock".into())
            }
            fn driver_version(&self) -> crate::types::AlpacaResult<String> {
                Ok(env!("CARGO_PKG_VERSION").into())
            }
            fn interface_version(&self) -> crate::types::AlpacaResult<i32> {
                Ok($version)
            }
            fn name(&self) -> crate::types::AlpacaResult<String> {
                Ok(self.$name_field.clone())
            }
            fn supported_actions(&self) -> crate::types::AlpacaResult<Vec<String>> {
                Ok(vec![])
            }
            fn device_state(
                &self,
            ) -> crate::types::AlpacaResult<Vec<crate::device::common::DeviceStateItem>> {
                #[allow(clippy::redundant_closure_call)]
                ($state_fn)(self)
            }
        }
    };
    // Variant with string literals for name/unique_id (most devices)
    (
        $struct_name:ty,
        name: $name:literal,
        unique_id: $unique_id:literal,
        device_type: $device_type:expr,
        interface_version: $version:literal,
        device_state: $state_fn:expr
    ) => {
        impl crate::device::Device for $struct_name {
            fn static_name(&self) -> &str {
                $name
            }
            fn unique_id(&self) -> &str {
                $unique_id
            }
            fn device_type(&self) -> crate::types::DeviceType {
                $device_type
            }
            fn connected(&self) -> crate::types::AlpacaResult<bool> {
                Ok(*self.connected.lock().unwrap())
            }
            fn set_connected(&self, v: bool) -> crate::types::AlpacaResult<()> {
                *self.connected.lock().unwrap() = v;
                Ok(())
            }
            fn connecting(&self) -> crate::types::AlpacaResult<bool> {
                Ok(false)
            }
            fn connect(&self) -> crate::types::AlpacaResult<()> {
                *self.connected.lock().unwrap() = true;
                Ok(())
            }
            fn disconnect(&self) -> crate::types::AlpacaResult<()> {
                *self.connected.lock().unwrap() = false;
                Ok(())
            }
            fn description(&self) -> crate::types::AlpacaResult<String> {
                Ok(format!("{} for ConformU testing", $name))
            }
            fn driver_info(&self) -> crate::types::AlpacaResult<String> {
                Ok("ascom-alpaca-core mock".into())
            }
            fn driver_version(&self) -> crate::types::AlpacaResult<String> {
                Ok(env!("CARGO_PKG_VERSION").into())
            }
            fn interface_version(&self) -> crate::types::AlpacaResult<i32> {
                Ok($version)
            }
            fn name(&self) -> crate::types::AlpacaResult<String> {
                Ok(($name).into())
            }
            fn supported_actions(&self) -> crate::types::AlpacaResult<Vec<String>> {
                Ok(vec![])
            }
            fn device_state(
                &self,
            ) -> crate::types::AlpacaResult<Vec<crate::device::common::DeviceStateItem>> {
                #[allow(clippy::redundant_closure_call)]
                ($state_fn)(self)
            }
        }
    };
}

pub mod camera;
pub mod cover_calibrator;
pub mod dome;
pub mod filter_wheel;
pub mod focuser;
pub mod observing_conditions;
pub mod rotator;
pub mod safety_monitor;
pub mod switch;
pub mod telescope;
