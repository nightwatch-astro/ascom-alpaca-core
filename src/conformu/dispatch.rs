use std::collections::HashMap;

use crate::device::Device;
use crate::registry::DeviceRegistry;
use crate::types::params::normalize_params;
use crate::types::{AlpacaError, AlpacaResponse, DeviceType, MethodResponse};

/// Parsed Alpaca API request.
pub struct AlpacaRequest {
    pub device_type: DeviceType,
    pub device_number: u32,
    pub method: String,
    pub params: HashMap<String, String>,
    pub is_put: bool,
}

/// Parse an Alpaca device API URL path.
/// Expected format: /api/v1/{device_type}/{device_number}/{method}
pub fn parse_device_path(path: &str) -> Option<(DeviceType, u32, String)> {
    let path = path.trim_start_matches('/');
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() < 5 || parts[0] != "api" || parts[1] != "v1" {
        return None;
    }
    let device_type = DeviceType::from_path(parts[2])?;
    let device_number: u32 = parts[3].parse().ok()?;
    let method = parts[4].to_lowercase();
    Some((device_type, device_number, method))
}

/// Parse URL query string into key-value pairs.
pub fn parse_query(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter(|s| !s.is_empty())
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next().unwrap_or("");
            Some((key.to_string(), url_decode(value)))
        })
        .collect()
}

fn url_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    result
}

/// Extract client transaction ID from normalized params.
fn client_tx(params: &HashMap<String, String>) -> u32 {
    params
        .get("clienttransactionid")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

/// Dispatch a device API request and return the JSON response body + HTTP status.
pub fn dispatch_request(
    registry: &DeviceRegistry,
    req: &AlpacaRequest,
    server_tx: u32,
) -> (String, u16) {
    let params = normalize_params(req.params.clone());
    let ctx = client_tx(&params);

    // Look up the device
    let device = match registry.get_device(req.device_type, req.device_number) {
        Ok(d) => d,
        Err(_) => {
            return (
                serde_json::to_string(&serde_json::json!({
                    "Value": null,
                    "ErrorNumber": 0,
                    "ErrorMessage": format!("Device not found: {} {}", req.device_type, req.device_number),
                    "ClientTransactionID": ctx,
                    "ServerTransactionID": server_tx,
                }))
                .unwrap(),
                400,
            );
        }
    };

    // Dispatch common Device methods first
    if let Some(resp) = dispatch_common(device, &req.method, &params, req.is_put, ctx, server_tx) {
        return (resp, 200);
    }

    // Dispatch device-type-specific methods
    let json = dispatch_device_specific(registry, req, &params, ctx, server_tx);
    (json, 200)
}

fn dispatch_common(
    device: &dyn Device,
    method: &str,
    _params: &HashMap<String, String>,
    is_put: bool,
    ctx: u32,
    stx: u32,
) -> Option<String> {
    match (method, is_put) {
        ("connected", false) => Some(respond_val(device.connected(), ctx, stx)),
        ("connected", true) => {
            let val = _params.get("connected").map(|v| v == "true" || v == "True").unwrap_or(false);
            Some(respond_void(device.set_connected(val), ctx, stx))
        }
        ("connecting", false) => Some(respond_val(device.connecting(), ctx, stx)),
        ("connect", true) => Some(respond_void(device.connect(), ctx, stx)),
        ("disconnect", true) => Some(respond_void(device.disconnect(), ctx, stx)),
        ("description", false) => Some(respond_val(device.description(), ctx, stx)),
        ("driverinfo", false) => Some(respond_val(device.driver_info(), ctx, stx)),
        ("driverversion", false) => Some(respond_val(device.driver_version(), ctx, stx)),
        ("interfaceversion", false) => Some(respond_val(device.interface_version(), ctx, stx)),
        ("name", false) => Some(respond_val(device.name(), ctx, stx)),
        ("supportedactions", false) => Some(respond_val(device.supported_actions(), ctx, stx)),
        ("action", true) => {
            let action_name = _params.get("action").cloned().unwrap_or_default();
            let action_params = _params.get("parameters").cloned().unwrap_or_default();
            Some(respond_val(device.action(&action_name, &action_params), ctx, stx))
        }
        ("commandblind", true) => {
            let cmd = _params.get("command").cloned().unwrap_or_default();
            let raw = _params.get("raw").map(|v| v == "true" || v == "True").unwrap_or(false);
            Some(respond_void(device.command_blind(&cmd, raw), ctx, stx))
        }
        ("commandbool", true) => {
            let cmd = _params.get("command").cloned().unwrap_or_default();
            let raw = _params.get("raw").map(|v| v == "true" || v == "True").unwrap_or(false);
            Some(respond_val(device.command_bool(&cmd, raw), ctx, stx))
        }
        ("commandstring", true) => {
            let cmd = _params.get("command").cloned().unwrap_or_default();
            let raw = _params.get("raw").map(|v| v == "true" || v == "True").unwrap_or(false);
            Some(respond_val(device.command_string(&cmd, raw), ctx, stx))
        }
        ("devicestate", false) => Some(respond_val(device.device_state(), ctx, stx)),
        _ => None,
    }
}

fn dispatch_device_specific(
    registry: &DeviceRegistry,
    req: &AlpacaRequest,
    params: &HashMap<String, String>,
    ctx: u32,
    stx: u32,
) -> String {
    match req.device_type {
        DeviceType::SafetyMonitor => dispatch_safety_monitor(registry, req.device_number, &req.method, params, req.is_put, ctx, stx),
        DeviceType::Camera => dispatch_camera(registry, req.device_number, &req.method, params, req.is_put, ctx, stx),
        DeviceType::Switch => dispatch_switch(registry, req.device_number, &req.method, params, req.is_put, ctx, stx),
        DeviceType::CoverCalibrator => dispatch_cover_calibrator(registry, req.device_number, &req.method, params, req.is_put, ctx, stx),
        DeviceType::Dome => dispatch_dome(registry, req.device_number, &req.method, params, req.is_put, ctx, stx),
        DeviceType::FilterWheel => dispatch_filter_wheel(registry, req.device_number, &req.method, params, req.is_put, ctx, stx),
        DeviceType::Focuser => dispatch_focuser(registry, req.device_number, &req.method, params, req.is_put, ctx, stx),
        DeviceType::ObservingConditions => dispatch_observing_conditions(registry, req.device_number, &req.method, params, req.is_put, ctx, stx),
        DeviceType::Rotator => dispatch_rotator(registry, req.device_number, &req.method, params, req.is_put, ctx, stx),
        DeviceType::Telescope => dispatch_telescope(registry, req.device_number, &req.method, params, req.is_put, ctx, stx),
    }
}

// --- Helper functions for building responses ---

fn respond_val<T: serde::Serialize>(result: Result<T, AlpacaError>, ctx: u32, stx: u32) -> String {
    match result {
        Ok(val) => serde_json::to_string(&AlpacaResponse::ok(val).with_transaction(ctx, stx)).unwrap(),
        Err(e) => {
            let resp = AlpacaResponse::<serde_json::Value>::from_error(e).with_transaction(ctx, stx);
            serde_json::to_string(&resp).unwrap()
        }
    }
}

fn respond_void(result: Result<(), AlpacaError>, ctx: u32, stx: u32) -> String {
    match result {
        Ok(()) => serde_json::to_string(&MethodResponse::ok().with_transaction(ctx, stx)).unwrap(),
        Err(e) => serde_json::to_string(&MethodResponse::from_error(e).with_transaction(ctx, stx)).unwrap(),
    }
}

fn param_i32(params: &HashMap<String, String>, key: &str) -> i32 {
    params.get(key).and_then(|v| v.parse().ok()).unwrap_or(0)
}

fn param_f64(params: &HashMap<String, String>, key: &str) -> f64 {
    params.get(key).and_then(|v| v.parse().ok()).unwrap_or(0.0)
}

fn param_bool(params: &HashMap<String, String>, key: &str) -> bool {
    params.get(key).map(|v| v == "true" || v == "True").unwrap_or(false)
}

fn not_implemented_response(method: &str, ctx: u32, stx: u32) -> String {
    respond_val::<bool>(Err(AlpacaError::NotImplemented(format!("{method} not implemented"))), ctx, stx)
}

// --- Per-device-type dispatch ---

fn dispatch_safety_monitor(registry: &DeviceRegistry, num: u32, method: &str, _params: &HashMap<String, String>, _is_put: bool, ctx: u32, stx: u32) -> String {
    let dev = match registry.get_safety_monitor(num) { Ok(d) => d, Err(_) => return not_implemented_response(method, ctx, stx) };
    match method {
        "issafe" => respond_val(dev.is_safe(), ctx, stx),
        _ => not_implemented_response(method, ctx, stx),
    }
}

fn dispatch_camera(registry: &DeviceRegistry, num: u32, method: &str, params: &HashMap<String, String>, is_put: bool, ctx: u32, stx: u32) -> String {
    let dev = match registry.get_camera(num) { Ok(d) => d, Err(_) => return not_implemented_response(method, ctx, stx) };
    match (method, is_put) {
        ("camerastate", false) => respond_val(dev.camera_state(), ctx, stx),
        ("cameraxsize", false) => respond_val(dev.camera_xsize(), ctx, stx),
        ("cameraysize", false) => respond_val(dev.camera_ysize(), ctx, stx),
        ("maxadu", false) => respond_val(dev.max_adu(), ctx, stx),
        ("sensorname", false) => respond_val(dev.sensor_name(), ctx, stx),
        ("sensortype", false) => respond_val(dev.sensor_type(), ctx, stx),
        ("pixelsizex", false) => respond_val(dev.pixel_size_x(), ctx, stx),
        ("pixelsizey", false) => respond_val(dev.pixel_size_y(), ctx, stx),
        ("binx", false) => respond_val(dev.bin_x(), ctx, stx),
        ("binx", true) => respond_void(dev.set_bin_x(param_i32(params, "binx")), ctx, stx),
        ("biny", false) => respond_val(dev.bin_y(), ctx, stx),
        ("biny", true) => respond_void(dev.set_bin_y(param_i32(params, "biny")), ctx, stx),
        ("maxbinx", false) => respond_val(dev.max_bin_x(), ctx, stx),
        ("maxbiny", false) => respond_val(dev.max_bin_y(), ctx, stx),
        ("canasymmetricbin", false) => respond_val(dev.can_asymmetric_bin(), ctx, stx),
        ("startx", false) => respond_val(dev.start_x(), ctx, stx),
        ("startx", true) => respond_void(dev.set_start_x(param_i32(params, "startx")), ctx, stx),
        ("starty", false) => respond_val(dev.start_y(), ctx, stx),
        ("starty", true) => respond_void(dev.set_start_y(param_i32(params, "starty")), ctx, stx),
        ("numx", false) => respond_val(dev.num_x(), ctx, stx),
        ("numx", true) => respond_void(dev.set_num_x(param_i32(params, "numx")), ctx, stx),
        ("numy", false) => respond_val(dev.num_y(), ctx, stx),
        ("numy", true) => respond_void(dev.set_num_y(param_i32(params, "numy")), ctx, stx),
        ("gain", false) => respond_val(dev.gain(), ctx, stx),
        ("gain", true) => respond_void(dev.set_gain(param_i32(params, "gain")), ctx, stx),
        ("gainmin", false) => respond_val(dev.gain_min(), ctx, stx),
        ("gainmax", false) => respond_val(dev.gain_max(), ctx, stx),
        ("gains", false) => respond_val(dev.gains(), ctx, stx),
        ("offset", false) => respond_val(dev.offset(), ctx, stx),
        ("offset", true) => respond_void(dev.set_offset(param_i32(params, "offset")), ctx, stx),
        ("offsetmin", false) => respond_val(dev.offset_min(), ctx, stx),
        ("offsetmax", false) => respond_val(dev.offset_max(), ctx, stx),
        ("offsets", false) => respond_val(dev.offsets(), ctx, stx),
        ("exposuremin", false) => respond_val(dev.exposure_min(), ctx, stx),
        ("exposuremax", false) => respond_val(dev.exposure_max(), ctx, stx),
        ("exposureresolution", false) => respond_val(dev.exposure_resolution(), ctx, stx),
        ("imageready", false) => respond_val(dev.image_ready(), ctx, stx),
        ("lastexposureduration", false) => respond_val(dev.last_exposure_duration(), ctx, stx),
        ("lastexposurestarttime", false) => respond_val(dev.last_exposure_start_time(), ctx, stx),
        ("startexposure", true) => {
            let duration = param_f64(params, "duration");
            let light = param_bool(params, "light");
            respond_void(dev.start_exposure(duration, light), ctx, stx)
        }
        ("stopexposure", true) => respond_void(dev.stop_exposure(), ctx, stx),
        ("imagearray" | "imagearrayvariant", false) => {
            match dev.image_array() {
                Ok(data) => {
                    let resp = data.to_response();
                    serde_json::to_string(&serde_json::json!({
                        "Type": resp.image_type,
                        "Rank": resp.rank,
                        "Value": resp.value,
                        "ErrorNumber": 0,
                        "ErrorMessage": "",
                        "ClientTransactionID": ctx,
                        "ServerTransactionID": stx,
                    })).unwrap()
                }
                Err(e) => respond_val::<serde_json::Value>(Err(e), ctx, stx),
            }
        }
        ("abortexposure", true) => respond_void(dev.abort_exposure(), ctx, stx),
        ("canabortexposure", false) => respond_val(dev.can_abort_exposure(), ctx, stx),
        ("canstopexposure", false) => respond_val(dev.can_stop_exposure(), ctx, stx),
        ("cooleron", false) => respond_val(dev.cooler_on(), ctx, stx),
        ("cooleron", true) => respond_void(dev.set_cooler_on(param_bool(params, "cooleron")), ctx, stx),
        ("coolerpower", false) => respond_val(dev.cooler_power(), ctx, stx),
        ("ccdtemperature", false) => respond_val(dev.ccd_temperature(), ctx, stx),
        ("heatsinktemperature", false) => respond_val(dev.heat_sink_temperature(), ctx, stx),
        ("setccdtemperature", false) => respond_val(dev.set_ccd_temperature(), ctx, stx),
        ("setccdtemperature", true) => respond_void(dev.set_set_ccd_temperature(param_f64(params, "setccdtemperature")), ctx, stx),
        ("cansetccdtemperature", false) => respond_val(dev.can_set_ccd_temperature(), ctx, stx),
        ("cangetcoolerpower", false) => respond_val(dev.can_get_cooler_power(), ctx, stx),
        ("canpulseguide", false) => respond_val(dev.can_pulse_guide(), ctx, stx),
        ("pulseguide", true) => {
            let dir_i = param_i32(params, "direction");
            let direction = match dir_i {
                0 => crate::camera::GuideDirection::North,
                1 => crate::camera::GuideDirection::South,
                2 => crate::camera::GuideDirection::East,
                3 => crate::camera::GuideDirection::West,
                _ => return respond_val::<bool>(Err(AlpacaError::InvalidValue(format!("Unknown guide direction: {dir_i}"))), ctx, stx),
            };
            respond_void(dev.pulse_guide(direction, param_i32(params, "duration")), ctx, stx)
        }
        ("ispulseguiding", false) => respond_val(dev.is_pulse_guiding(), ctx, stx),
        ("percentcompleted", false) => respond_val(dev.percent_completed(), ctx, stx),
        ("readoutmode", false) => respond_val(dev.readout_mode(), ctx, stx),
        ("readoutmode", true) => respond_void(dev.set_readout_mode(param_i32(params, "readoutmode")), ctx, stx),
        ("readoutmodes", false) => respond_val(dev.readout_modes(), ctx, stx),
        ("canfastreadout", false) => respond_val(dev.can_fast_readout(), ctx, stx),
        ("fastreadout", false) => respond_val(dev.fast_readout(), ctx, stx),
        ("fastreadout", true) => respond_void(dev.set_fast_readout(param_bool(params, "fastreadout")), ctx, stx),
        ("electronsperadu", false) => respond_val(dev.electrons_per_adu(), ctx, stx),
        ("fullwellcapacity", false) => respond_val(dev.full_well_capacity(), ctx, stx),
        ("hasshutter", false) => respond_val(dev.has_shutter(), ctx, stx),
        ("bayeroffsetx", false) => respond_val(dev.bayer_offset_x(), ctx, stx),
        ("bayeroffsety", false) => respond_val(dev.bayer_offset_y(), ctx, stx),
        ("subexposureduration", false) => respond_val(dev.sub_exposure_duration(), ctx, stx),
        ("subexposureduration", true) => respond_void(dev.set_sub_exposure_duration(param_f64(params, "subexposureduration")), ctx, stx),
        _ => not_implemented_response(method, ctx, stx),
    }
}

fn dispatch_switch(registry: &DeviceRegistry, num: u32, method: &str, params: &HashMap<String, String>, is_put: bool, ctx: u32, stx: u32) -> String {
    let dev = match registry.get_switch(num) { Ok(d) => d, Err(_) => return not_implemented_response(method, ctx, stx) };
    // Switch ID must be parsed as i32 first — negative IDs are invalid
    let raw_id = param_i32(params, "id");
    if raw_id < 0 {
        return respond_val::<bool>(Err(AlpacaError::InvalidValue(format!("Switch ID {raw_id} is negative"))), ctx, stx);
    }
    let id = raw_id as u32;
    match (method, is_put) {
        ("maxswitch", false) => respond_val(dev.max_switch(), ctx, stx),
        ("canwrite", false) => respond_val(dev.can_write(id), ctx, stx),
        ("getswitch", false) => respond_val(dev.get_switch(id), ctx, stx),
        ("setswitch", true) => respond_void(dev.set_switch(id, param_bool(params, "state")), ctx, stx),
        ("getswitchvalue", false) => respond_val(dev.get_switch_value(id), ctx, stx),
        ("setswitchvalue", true) => respond_void(dev.set_switch_value(id, param_f64(params, "value")), ctx, stx),
        ("getswitchname", false) => respond_val(dev.get_switch_name(id), ctx, stx),
        ("setswitchname", true) => { let name = params.get("name").cloned().unwrap_or_default(); respond_void(dev.set_switch_name(id, &name), ctx, stx) }
        ("getswitchdescription", false) => respond_val(dev.get_switch_description(id), ctx, stx),
        ("minswitchvalue", false) => respond_val(dev.min_switch_value(id), ctx, stx),
        ("maxswitchvalue", false) => respond_val(dev.max_switch_value(id), ctx, stx),
        ("switchstep", false) => respond_val(dev.switch_step(id), ctx, stx),
        ("canasync", false) => respond_val(dev.can_async(id), ctx, stx),
        ("setasync", true) => respond_void(dev.set_async(id, param_bool(params, "state")), ctx, stx),
        ("setasyncvalue", true) => respond_void(dev.set_async_value(id, param_f64(params, "value")), ctx, stx),
        ("cancelasync", true) => respond_void(dev.cancel_async(id), ctx, stx),
        ("statechangecomplete", false) => respond_val(dev.state_change_complete(id), ctx, stx),
        _ => not_implemented_response(method, ctx, stx),
    }
}

fn dispatch_cover_calibrator(registry: &DeviceRegistry, num: u32, method: &str, params: &HashMap<String, String>, is_put: bool, ctx: u32, stx: u32) -> String {
    let dev = match registry.get_cover_calibrator(num) { Ok(d) => d, Err(_) => return not_implemented_response(method, ctx, stx) };
    match (method, is_put) {
        ("brightness", false) => respond_val(dev.brightness(), ctx, stx),
        ("maxbrightness", false) => respond_val(dev.max_brightness(), ctx, stx),
        ("calibratorstate", false) => respond_val(dev.calibrator_state(), ctx, stx),
        ("coverstate", false) => respond_val(dev.cover_state(), ctx, stx),
        ("calibratoron", true) => respond_void(dev.calibrator_on(param_i32(params, "brightness")), ctx, stx),
        ("calibratoroff", true) => respond_void(dev.calibrator_off(), ctx, stx),
        ("opencover", true) => respond_void(dev.open_cover(), ctx, stx),
        ("closecover", true) => respond_void(dev.close_cover(), ctx, stx),
        ("haltcover", true) => respond_void(dev.halt_cover(), ctx, stx),
        ("calibratorchanging", false) => respond_val(dev.calibrator_changing(), ctx, stx),
        ("covermoving", false) => respond_val(dev.cover_moving(), ctx, stx),
        _ => not_implemented_response(method, ctx, stx),
    }
}

fn dispatch_dome(registry: &DeviceRegistry, num: u32, method: &str, params: &HashMap<String, String>, is_put: bool, ctx: u32, stx: u32) -> String {
    let dev = match registry.get_dome(num) { Ok(d) => d, Err(_) => return not_implemented_response(method, ctx, stx) };
    match (method, is_put) {
        ("altitude", false) => respond_val(dev.altitude(), ctx, stx),
        ("azimuth", false) => respond_val(dev.azimuth(), ctx, stx),
        ("athome", false) => respond_val(dev.at_home(), ctx, stx),
        ("atpark", false) => respond_val(dev.at_park(), ctx, stx),
        ("shutterstatus", false) => respond_val(dev.shutter_status(), ctx, stx),
        ("slaved", false) => respond_val(dev.slaved(), ctx, stx),
        ("slaved", true) => respond_void(dev.set_slaved(param_bool(params, "slaved")), ctx, stx),
        ("slewing", false) => respond_val(dev.slewing(), ctx, stx),
        ("canfindhome", false) => respond_val(dev.can_find_home(), ctx, stx),
        ("canpark", false) => respond_val(dev.can_park(), ctx, stx),
        ("cansetaltitude", false) => respond_val(dev.can_set_altitude(), ctx, stx),
        ("cansetazimuth", false) => respond_val(dev.can_set_azimuth(), ctx, stx),
        ("cansetpark", false) => respond_val(dev.can_set_park(), ctx, stx),
        ("cansetshutter", false) => respond_val(dev.can_set_shutter(), ctx, stx),
        ("canslave", false) => respond_val(dev.can_slave(), ctx, stx),
        ("cansyncazimuth", false) => respond_val(dev.can_sync_azimuth(), ctx, stx),
        ("slewtoazimuth", true) => respond_void(dev.slew_to_azimuth(param_f64(params, "azimuth")), ctx, stx),
        ("slewtoaltitude", true) => respond_void(dev.slew_to_altitude(param_f64(params, "altitude")), ctx, stx),
        ("openshutter", true) => respond_void(dev.open_shutter(), ctx, stx),
        ("closeshutter", true) => respond_void(dev.close_shutter(), ctx, stx),
        ("park", true) => respond_void(dev.park(), ctx, stx),
        ("setpark", true) => respond_void(dev.set_park(), ctx, stx),
        ("findhome", true) => respond_void(dev.find_home(), ctx, stx),
        ("abortslew", true) => respond_void(dev.abort_slew(), ctx, stx),
        ("synctoazimuth", true) => respond_void(dev.sync_to_azimuth(param_f64(params, "azimuth")), ctx, stx),
        _ => not_implemented_response(method, ctx, stx),
    }
}

fn dispatch_filter_wheel(registry: &DeviceRegistry, num: u32, method: &str, params: &HashMap<String, String>, is_put: bool, ctx: u32, stx: u32) -> String {
    let dev = match registry.get_filter_wheel(num) { Ok(d) => d, Err(_) => return not_implemented_response(method, ctx, stx) };
    match (method, is_put) {
        ("position", false) => respond_val(dev.position(), ctx, stx),
        ("position", true) => respond_void(dev.set_position(param_i32(params, "position")), ctx, stx),
        ("names", false) => respond_val(dev.names(), ctx, stx),
        ("focusoffsets", false) => respond_val(dev.focus_offsets(), ctx, stx),
        _ => not_implemented_response(method, ctx, stx),
    }
}

fn dispatch_focuser(registry: &DeviceRegistry, num: u32, method: &str, params: &HashMap<String, String>, is_put: bool, ctx: u32, stx: u32) -> String {
    let dev = match registry.get_focuser(num) { Ok(d) => d, Err(_) => return not_implemented_response(method, ctx, stx) };
    match (method, is_put) {
        ("absolute", false) => respond_val(dev.absolute(), ctx, stx),
        ("ismoving", false) => respond_val(dev.is_moving(), ctx, stx),
        ("maxincrement", false) => respond_val(dev.max_increment(), ctx, stx),
        ("maxstep", false) => respond_val(dev.max_step(), ctx, stx),
        ("position", false) => respond_val(dev.position(), ctx, stx),
        ("stepsize", false) => respond_val(dev.step_size(), ctx, stx),
        ("temperature", false) => respond_val(dev.temperature(), ctx, stx),
        ("tempcomp", false) => respond_val(dev.temp_comp(), ctx, stx),
        ("tempcomp", true) => respond_void(dev.set_temp_comp(param_bool(params, "tempcomp")), ctx, stx),
        ("tempcompavailable", false) => respond_val(dev.temp_comp_available(), ctx, stx),
        ("halt", true) => respond_void(dev.halt(), ctx, stx),
        ("move", true) => respond_void(dev.r#move(param_i32(params, "position")), ctx, stx),
        _ => not_implemented_response(method, ctx, stx),
    }
}

fn dispatch_observing_conditions(registry: &DeviceRegistry, num: u32, method: &str, params: &HashMap<String, String>, is_put: bool, ctx: u32, stx: u32) -> String {
    let dev = match registry.get_observing_conditions(num) { Ok(d) => d, Err(_) => return not_implemented_response(method, ctx, stx) };
    match (method, is_put) {
        ("cloudcover", false) => respond_val(dev.cloud_cover(), ctx, stx),
        ("dewpoint", false) => respond_val(dev.dew_point(), ctx, stx),
        ("humidity", false) => respond_val(dev.humidity(), ctx, stx),
        ("pressure", false) => respond_val(dev.pressure(), ctx, stx),
        ("rainrate", false) => respond_val(dev.rain_rate(), ctx, stx),
        ("skybrightness", false) => respond_val(dev.sky_brightness(), ctx, stx),
        ("skyquality", false) => respond_val(dev.sky_quality(), ctx, stx),
        ("skytemperature", false) => respond_val(dev.sky_temperature(), ctx, stx),
        ("starfwhm", false) => respond_val(dev.star_fwhm(), ctx, stx),
        ("temperature", false) => respond_val(dev.temperature(), ctx, stx),
        ("winddirection", false) => respond_val(dev.wind_direction(), ctx, stx),
        ("windgust", false) => respond_val(dev.wind_gust(), ctx, stx),
        ("windspeed", false) => respond_val(dev.wind_speed(), ctx, stx),
        ("averageperiod", false) => respond_val(dev.average_period(), ctx, stx),
        ("averageperiod", true) => respond_void(dev.set_average_period(param_f64(params, "averageperiod")), ctx, stx),
        ("sensordescription", false) => {
            let sensor = params.get("sensorname").cloned().unwrap_or_default();
            respond_val(dev.sensor_description(&sensor), ctx, stx)
        }
        ("timesincelastupdate", false) => {
            let sensor = params.get("sensorname").cloned().unwrap_or_default();
            respond_val(dev.time_of_latest_update(&sensor), ctx, stx)
        }
        ("refresh", true) => respond_void(dev.refresh(), ctx, stx),
        _ => not_implemented_response(method, ctx, stx),
    }
}

fn dispatch_rotator(registry: &DeviceRegistry, num: u32, method: &str, params: &HashMap<String, String>, is_put: bool, ctx: u32, stx: u32) -> String {
    let dev = match registry.get_rotator(num) { Ok(d) => d, Err(_) => return not_implemented_response(method, ctx, stx) };
    match (method, is_put) {
        ("canreverse", false) => respond_val(dev.can_reverse(), ctx, stx),
        ("ismoving", false) => respond_val(dev.is_moving(), ctx, stx),
        ("mechanicalposition", false) => respond_val(dev.mechanical_position(), ctx, stx),
        ("position", false) => respond_val(dev.position(), ctx, stx),
        ("reverse", false) => respond_val(dev.reverse(), ctx, stx),
        ("reverse", true) => respond_void(dev.set_reverse(param_bool(params, "reverse")), ctx, stx),
        ("stepsize", false) => respond_val(dev.step_size(), ctx, stx),
        ("targetposition", false) => respond_val(dev.target_position(), ctx, stx),
        ("halt", true) => respond_void(dev.halt(), ctx, stx),
        ("move", true) => respond_void(dev.r#move(param_f64(params, "position")), ctx, stx),
        ("moveabsolute", true) => respond_void(dev.move_absolute(param_f64(params, "position")), ctx, stx),
        ("movemechanical", true) => respond_void(dev.move_mechanical(param_f64(params, "position")), ctx, stx),
        ("sync", true) => respond_void(dev.sync(param_f64(params, "position")), ctx, stx),
        _ => not_implemented_response(method, ctx, stx),
    }
}

fn dispatch_telescope(registry: &DeviceRegistry, num: u32, method: &str, params: &HashMap<String, String>, is_put: bool, ctx: u32, stx: u32) -> String {
    use crate::telescope::{DriveRate, SideOfPier};
    use crate::types::GuideDirection;

    let dev = match registry.get_telescope(num) { Ok(d) => d, Err(_) => return not_implemented_response(method, ctx, stx) };
    match (method, is_put) {
        // --- Position & coordinates ---
        ("altitude", false) => respond_val(dev.altitude(), ctx, stx),
        ("azimuth", false) => respond_val(dev.azimuth(), ctx, stx),
        ("rightascension", false) => respond_val(dev.right_ascension(), ctx, stx),
        ("declination", false) => respond_val(dev.declination(), ctx, stx),
        ("targetrightascension", false) => respond_val(dev.target_right_ascension(), ctx, stx),
        ("targetrightascension", true) => respond_void(dev.set_target_right_ascension(param_f64(params, "targetrightascension")), ctx, stx),
        ("targetdeclination", false) => respond_val(dev.target_declination(), ctx, stx),
        ("targetdeclination", true) => respond_void(dev.set_target_declination(param_f64(params, "targetdeclination")), ctx, stx),
        ("siderealtime", false) => respond_val(dev.sidereal_time(), ctx, stx),

        // --- Slewing ---
        ("slewing", false) => respond_val(dev.slewing(), ctx, stx),
        ("slewtocoordinates", true) => respond_void(dev.slew_to_coordinates(param_f64(params, "rightascension"), param_f64(params, "declination")), ctx, stx),
        ("slewtocoordinatesasync", true) => respond_void(dev.slew_to_coordinates_async(param_f64(params, "rightascension"), param_f64(params, "declination")), ctx, stx),
        ("slewtoaltaz", true) => respond_void(dev.slew_to_alt_az(param_f64(params, "azimuth"), param_f64(params, "altitude")), ctx, stx),
        ("slewtoaltazasync", true) => respond_void(dev.slew_to_alt_az_async(param_f64(params, "azimuth"), param_f64(params, "altitude")), ctx, stx),
        ("slewtotarget", true) => respond_void(dev.slew_to_target(), ctx, stx),
        ("slewtotargetasync", true) => respond_void(dev.slew_to_target_async(), ctx, stx),
        ("abortslew", true) => respond_void(dev.abort_slew(), ctx, stx),
        ("moveaxis", true) => respond_void(dev.move_axis(param_i32(params, "axis"), param_f64(params, "rate")), ctx, stx),
        ("destinationsideofpier", false) => respond_val(dev.destination_side_of_pier(param_f64(params, "rightascension"), param_f64(params, "declination")), ctx, stx),

        // --- Tracking ---
        ("tracking", false) => respond_val(dev.tracking(), ctx, stx),
        ("tracking", true) => respond_void(dev.set_tracking(param_bool(params, "tracking")), ctx, stx),
        ("trackingrate", false) => respond_val(dev.tracking_rate(), ctx, stx),
        ("trackingrate", true) => {
            let rate_i = param_i32(params, "trackingrate");
            let rate = match rate_i {
                0 => DriveRate::Sidereal,
                1 => DriveRate::Lunar,
                2 => DriveRate::Solar,
                3 => DriveRate::King,
                _ => return respond_val::<bool>(Err(AlpacaError::InvalidValue(format!("Unknown tracking rate: {rate_i}"))), ctx, stx),
            };
            respond_void(dev.set_tracking_rate(rate), ctx, stx)
        }
        ("trackingrates", false) => respond_val(dev.tracking_rates(), ctx, stx),
        ("rightascensionrate", false) => respond_val(dev.right_ascension_rate(), ctx, stx),
        ("rightascensionrate", true) => respond_void(dev.set_right_ascension_rate(param_f64(params, "rightascensionrate")), ctx, stx),
        ("declinationrate", false) => respond_val(dev.declination_rate(), ctx, stx),
        ("declinationrate", true) => respond_void(dev.set_declination_rate(param_f64(params, "declinationrate")), ctx, stx),

        // --- Parking ---
        ("athome", false) => respond_val(dev.at_home(), ctx, stx),
        ("atpark", false) => respond_val(dev.at_park(), ctx, stx),
        ("park", true) => respond_void(dev.park(), ctx, stx),
        ("unpark", true) => respond_void(dev.unpark(), ctx, stx),
        ("setpark", true) => respond_void(dev.set_park(), ctx, stx),
        ("findhome", true) => respond_void(dev.find_home(), ctx, stx),

        // --- Pulse guiding ---
        ("pulseguide", true) => {
            let dir_i = param_i32(params, "direction");
            let direction = match dir_i {
                0 => GuideDirection::North,
                1 => GuideDirection::South,
                2 => GuideDirection::East,
                3 => GuideDirection::West,
                _ => return respond_val::<bool>(Err(AlpacaError::InvalidValue(format!("Unknown guide direction: {dir_i}"))), ctx, stx),
            };
            respond_void(dev.pulse_guide(direction, param_i32(params, "duration")), ctx, stx)
        }
        ("ispulseguiding", false) => respond_val(dev.is_pulse_guiding(), ctx, stx),
        ("guideraterightascension", false) => respond_val(dev.guide_rate_right_ascension(), ctx, stx),
        ("guideraterightascension", true) => respond_void(dev.set_guide_rate_right_ascension(param_f64(params, "guideraterightascension")), ctx, stx),
        ("guideratedeclination", false) => respond_val(dev.guide_rate_declination(), ctx, stx),
        ("guideratedeclination", true) => respond_void(dev.set_guide_rate_declination(param_f64(params, "guideratedeclination")), ctx, stx),

        // --- Side of pier ---
        ("sideofpier", false) => respond_val(dev.side_of_pier(), ctx, stx),
        ("sideofpier", true) => {
            let side_i = param_i32(params, "sideofpier");
            let side = match side_i {
                0 => SideOfPier::East,
                1 => SideOfPier::West,
                -1 => SideOfPier::Unknown,
                _ => return respond_val::<bool>(Err(AlpacaError::InvalidValue(format!("Unknown side of pier: {side_i}"))), ctx, stx),
            };
            respond_void(dev.set_side_of_pier(side), ctx, stx)
        }

        // --- Site location ---
        ("siteelevation", false) => respond_val(dev.site_elevation(), ctx, stx),
        ("siteelevation", true) => respond_void(dev.set_site_elevation(param_f64(params, "siteelevation")), ctx, stx),
        ("sitelatitude", false) => respond_val(dev.site_latitude(), ctx, stx),
        ("sitelatitude", true) => respond_void(dev.set_site_latitude(param_f64(params, "sitelatitude")), ctx, stx),
        ("sitelongitude", false) => respond_val(dev.site_longitude(), ctx, stx),
        ("sitelongitude", true) => respond_void(dev.set_site_longitude(param_f64(params, "sitelongitude")), ctx, stx),
        ("utcdate", false) => respond_val(dev.utc_date(), ctx, stx),
        ("utcdate", true) => {
            let utc = params.get("utcdate").cloned().unwrap_or_default();
            respond_void(dev.set_utc_date(&utc), ctx, stx)
        }

        // --- Axis rates ---
        ("axisrates", false) => respond_val(dev.axis_rates(param_i32(params, "axis")), ctx, stx),
        ("canmoveaxis", false) => respond_val(dev.can_move_axis(param_i32(params, "axis")), ctx, stx),

        // --- Sync ---
        ("synctocoordinates", true) => respond_void(dev.sync_to_coordinates(param_f64(params, "rightascension"), param_f64(params, "declination")), ctx, stx),
        ("synctotarget", true) => respond_void(dev.sync_to_target(), ctx, stx),
        ("synctoaltaz", true) => respond_void(dev.sync_to_alt_az(param_f64(params, "azimuth"), param_f64(params, "altitude")), ctx, stx),

        // --- Capabilities ---
        ("alignmentmode", false) => respond_val(dev.alignment_mode(), ctx, stx),
        ("equatorialsystem", false) => respond_val(dev.equatorial_system(), ctx, stx),
        ("canfindhome", false) => respond_val(dev.can_find_home(), ctx, stx),
        ("canpark", false) => respond_val(dev.can_park(), ctx, stx),
        ("canpulseguide", false) => respond_val(dev.can_pulse_guide(), ctx, stx),
        ("canslew", false) => respond_val(dev.can_slew(), ctx, stx),
        ("canslewasync", false) => respond_val(dev.can_slew_async(), ctx, stx),
        ("canslewaltaz", false) => respond_val(dev.can_slew_alt_az(), ctx, stx),
        ("canslewaltazasync", false) => respond_val(dev.can_slew_alt_az_async(), ctx, stx),
        ("cansync", false) => respond_val(dev.can_sync(), ctx, stx),
        ("cansyncaltaz", false) => respond_val(dev.can_sync_alt_az(), ctx, stx),
        ("canunpark", false) => respond_val(dev.can_unpark(), ctx, stx),
        ("cansettracking", false) => respond_val(dev.can_set_tracking(), ctx, stx),
        ("cansetpark", false) => respond_val(dev.can_set_park(), ctx, stx),
        ("cansetpierside", false) => respond_val(dev.can_set_pier_side(), ctx, stx),
        ("cansetguiderates", false) => respond_val(dev.can_set_guide_rates(), ctx, stx),
        ("cansetdeclinationrate", false) => respond_val(dev.can_set_declination_rate(), ctx, stx),
        ("cansetrightascensionrate", false) => respond_val(dev.can_set_right_ascension_rate(), ctx, stx),
        ("doesrefraction", false) => respond_val(dev.does_refraction(), ctx, stx),
        ("doesrefraction", true) => respond_void(dev.set_does_refraction(param_bool(params, "doesrefraction")), ctx, stx),
        ("aperturearea", false) => respond_val(dev.aperture_area(), ctx, stx),
        ("aperturediameter", false) => respond_val(dev.aperture_diameter(), ctx, stx),
        ("focallength", false) => respond_val(dev.focal_length(), ctx, stx),
        _ => not_implemented_response(method, ctx, stx),
    }
}
