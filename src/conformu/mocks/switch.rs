use std::sync::Mutex;

use crate::device::Device;
use crate::switch::Switch;
use crate::types::{AlpacaError, AlpacaResult, DeviceType};

/// Per-channel configuration for mock switches.
struct SwitchChannel {
    name: &'static str,
    description: &'static str,
    min: f64,
    max: f64,
    step: f64,
    can_write: bool,
}

/// Default channels demonstrating all ASCOM switch types:
/// - Boolean (on/off): min=0, max=1, step=1
/// - Multi-state (selector): min=0, max=3, step=1 (4 positions)
/// - Analog (dimmer): min=0, max=100, step=0.5
const CHANNELS: &[SwitchChannel] = &[
    SwitchChannel {
        name: "Power",
        description: "Boolean on/off switch",
        min: 0.0,
        max: 1.0,
        step: 1.0,
        can_write: true,
    },
    SwitchChannel {
        name: "Selector",
        description: "Multi-state rotary selector (4 positions)",
        min: 0.0,
        max: 3.0,
        step: 1.0,
        can_write: true,
    },
    SwitchChannel {
        name: "Dimmer",
        description: "Analog dimmer (0-100%, 0.5 step)",
        min: 0.0,
        max: 100.0,
        step: 0.5,
        can_write: true,
    },
];

pub struct MockSwitch {
    connected: Mutex<bool>,
    values: Mutex<Vec<f64>>,
    names: Mutex<Vec<String>>,
}

impl Default for MockSwitch {
    fn default() -> Self {
        Self::new()
    }
}

impl MockSwitch {
    pub fn new() -> Self {
        let values = CHANNELS.iter().map(|ch| ch.min).collect();
        let names = CHANNELS.iter().map(|ch| ch.name.to_string()).collect();
        Self {
            connected: Mutex::new(false),
            values: Mutex::new(values),
            names: Mutex::new(names),
        }
    }

    fn validate_id(id: u32) -> AlpacaResult<&'static SwitchChannel> {
        CHANNELS.get(id as usize).ok_or_else(|| {
            AlpacaError::InvalidValue(format!(
                "Switch ID {id} out of range 0-{}",
                CHANNELS.len() - 1
            ))
        })
    }

    /// Clamp value to nearest valid step within channel range.
    fn clamp_to_step(ch: &SwitchChannel, value: f64) -> f64 {
        let clamped = value.clamp(ch.min, ch.max);
        if ch.step > 0.0 {
            let steps = ((clamped - ch.min) / ch.step).round();
            (ch.min + steps * ch.step).min(ch.max)
        } else {
            clamped
        }
    }
}

impl Device for MockSwitch {
    fn static_name(&self) -> &str { "Mock Switch" }
    fn unique_id(&self) -> &str { "mock-sw-001" }
    fn device_type(&self) -> DeviceType { DeviceType::Switch }
    fn connected(&self) -> AlpacaResult<bool> { Ok(*self.connected.lock().unwrap()) }
    fn set_connected(&self, v: bool) -> AlpacaResult<()> { *self.connected.lock().unwrap() = v; Ok(()) }
    fn connecting(&self) -> AlpacaResult<bool> { Ok(false) }
    fn connect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = true; Ok(()) }
    fn disconnect(&self) -> AlpacaResult<()> { *self.connected.lock().unwrap() = false; Ok(()) }
    fn description(&self) -> AlpacaResult<String> { Ok("Mock Switch with boolean, multi-state, and analog channels".into()) }
    fn driver_info(&self) -> AlpacaResult<String> { Ok("ascom-alpaca-core mock".into()) }
    fn driver_version(&self) -> AlpacaResult<String> { Ok(env!("CARGO_PKG_VERSION").into()) }
    fn interface_version(&self) -> AlpacaResult<i32> { Ok(3) }
    fn name(&self) -> AlpacaResult<String> { Ok("Mock Switch".into()) }
    fn supported_actions(&self) -> AlpacaResult<Vec<String>> { Ok(vec![]) }
    fn device_state(&self) -> AlpacaResult<Vec<crate::device::common::DeviceStateItem>> {
        use crate::device::common::DeviceStateItem;
        let values = self.values.lock().unwrap();
        let mut items = Vec::new();
        for (i, ch) in CHANNELS.iter().enumerate() {
            let val = values[i];
            let bool_val = val >= (ch.min + ch.max) / 2.0;
            items.push(DeviceStateItem {
                name: format!("GetSwitch{i}"),
                value: serde_json::json!(bool_val),
            });
            items.push(DeviceStateItem {
                name: format!("GetSwitchValue{i}"),
                value: serde_json::json!(val),
            });
            items.push(DeviceStateItem {
                name: format!("StateChangeComplete{i}"),
                value: serde_json::json!(true),
            });
        }
        Ok(items)
    }
}

impl Switch for MockSwitch {
    fn max_switch(&self) -> AlpacaResult<i32> { Ok(CHANNELS.len() as i32) }

    fn can_write(&self, id: u32) -> AlpacaResult<bool> {
        let ch = Self::validate_id(id)?;
        Ok(ch.can_write)
    }

    fn get_switch(&self, id: u32) -> AlpacaResult<bool> {
        Self::validate_id(id)?;
        let values = self.values.lock().unwrap();
        let val = values[id as usize];
        // Boolean interpretation: value >= midpoint of range
        let ch = &CHANNELS[id as usize];
        Ok(val >= (ch.min + ch.max) / 2.0)
    }

    fn set_switch(&self, id: u32, state: bool) -> AlpacaResult<()> {
        let ch = Self::validate_id(id)?;
        if !ch.can_write {
            return Err(AlpacaError::NotImplemented("Switch is read-only".into()));
        }
        let mut values = self.values.lock().unwrap();
        values[id as usize] = if state { ch.max } else { ch.min };
        Ok(())
    }

    fn get_switch_value(&self, id: u32) -> AlpacaResult<f64> {
        Self::validate_id(id)?;
        let values = self.values.lock().unwrap();
        Ok(values[id as usize])
    }

    fn set_switch_value(&self, id: u32, value: f64) -> AlpacaResult<()> {
        let ch = Self::validate_id(id)?;
        if !ch.can_write {
            return Err(AlpacaError::NotImplemented("Switch is read-only".into()));
        }
        if value < ch.min || value > ch.max {
            return Err(AlpacaError::InvalidValue(format!(
                "Value {value} out of range {}-{}",
                ch.min, ch.max
            )));
        }
        let mut values = self.values.lock().unwrap();
        values[id as usize] = Self::clamp_to_step(ch, value);
        Ok(())
    }

    fn get_switch_name(&self, id: u32) -> AlpacaResult<String> {
        Self::validate_id(id)?;
        let names = self.names.lock().unwrap();
        Ok(names[id as usize].clone())
    }

    fn set_switch_name(&self, id: u32, name: &str) -> AlpacaResult<()> {
        Self::validate_id(id)?;
        let mut names = self.names.lock().unwrap();
        names[id as usize] = name.to_string();
        Ok(())
    }

    fn get_switch_description(&self, id: u32) -> AlpacaResult<String> {
        let ch = Self::validate_id(id)?;
        Ok(ch.description.to_string())
    }

    fn min_switch_value(&self, id: u32) -> AlpacaResult<f64> {
        let ch = Self::validate_id(id)?;
        Ok(ch.min)
    }

    fn max_switch_value(&self, id: u32) -> AlpacaResult<f64> {
        let ch = Self::validate_id(id)?;
        Ok(ch.max)
    }

    fn switch_step(&self, id: u32) -> AlpacaResult<f64> {
        let ch = Self::validate_id(id)?;
        Ok(ch.step)
    }

    fn can_async(&self, id: u32) -> AlpacaResult<bool> {
        Self::validate_id(id)?;
        Ok(false)
    }

    fn set_async(&self, id: u32, _state: bool) -> AlpacaResult<()> {
        Self::validate_id(id)?;
        Err(AlpacaError::NotImplemented("Async not supported".into()))
    }

    fn set_async_value(&self, id: u32, _value: f64) -> AlpacaResult<()> {
        Self::validate_id(id)?;
        Err(AlpacaError::NotImplemented("Async not supported".into()))
    }

    fn cancel_async(&self, id: u32) -> AlpacaResult<()> {
        Self::validate_id(id)?;
        Err(AlpacaError::NotImplemented("Async not supported".into()))
    }

    fn state_change_complete(&self, id: u32) -> AlpacaResult<bool> {
        Self::validate_id(id)?;
        Err(AlpacaError::NotImplemented("Async not supported".into()))
    }
}
