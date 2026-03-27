//! ConformU test harness — serves mock ASCOM Alpaca devices over HTTP.
//!
//! Run: `cargo run --example conformu_harness --features conformu`
//! Then point ConformU at http://127.0.0.1:32888

use ascom_alpaca_core::conformu::dispatch::{parse_device_path, parse_query, AlpacaRequest};
use ascom_alpaca_core::conformu::management;
use ascom_alpaca_core::conformu::mocks;
use ascom_alpaca_core::registry::{DeviceRegistry, TransactionCounter};

use ascom_alpaca_core::camera::{Camera, SensorType};
use ascom_alpaca_core::conformu::mocks::camera::GainOffsetMode;
use ascom_alpaca_core::cover_calibrator::CoverCalibrator;
use ascom_alpaca_core::dome::Dome;
use ascom_alpaca_core::filter_wheel::FilterWheel;
use ascom_alpaca_core::focuser::Focuser;
use ascom_alpaca_core::observing_conditions::ObservingConditions;
use ascom_alpaca_core::rotator::Rotator;
use ascom_alpaca_core::safety_monitor::SafetyMonitor;
use ascom_alpaca_core::switch::Switch;
use ascom_alpaca_core::telescope::Telescope;

fn main() {
    let mut registry = DeviceRegistry::new();

    let sm: Box<dyn SafetyMonitor> = Box::new(mocks::safety_monitor::MockSafetyMonitor::new());
    let cam: Box<dyn Camera> = Box::new(mocks::camera::MockCamera::full_featured());
    let cam_color: Box<dyn Camera> = Box::new(mocks::camera::MockCamera::with_features_and_id(
        mocks::camera::CameraFeatures {
            cooler: true,
            pulse_guide: true,
            fast_readout: true,
            asymmetric_bin: true,
            gain_mode: GainOffsetMode::Named(vec![
                "Low".into(), "Medium".into(), "High".into(),
            ]),
            offset_mode: GainOffsetMode::Named(vec![
                "Normal".into(), "Low Noise".into(),
            ]),
            shutter: true,
            sub_exposure: true,
            sensor_type: SensorType::RGGB,
        },
        "mock-cam-002",
        "Mock Color Camera",
    ));
    let sw: Box<dyn Switch> = Box::new(mocks::switch::MockSwitch::new());
    let cc: Box<dyn CoverCalibrator> = Box::new(mocks::cover_calibrator::MockCoverCalibrator::new());
    let dome: Box<dyn Dome> = Box::new(mocks::dome::MockDome::new());
    let fw: Box<dyn FilterWheel> = Box::new(mocks::filter_wheel::MockFilterWheel::new());
    let foc: Box<dyn Focuser> = Box::new(mocks::focuser::MockFocuser::new());
    let oc: Box<dyn ObservingConditions> = Box::new(mocks::observing_conditions::MockObservingConditions::new());
    let rot: Box<dyn Rotator> = Box::new(mocks::rotator::MockRotator::new());
    let tel: Box<dyn Telescope> = Box::new(mocks::telescope::MockTelescope::new());

    registry.register(sm);
    registry.register(cam);
    registry.register(cam_color);
    registry.register(sw);
    registry.register(cc);
    registry.register(dome);
    registry.register(fw);
    registry.register(foc);
    registry.register(oc);
    registry.register(rot);
    registry.register(tel);

    let tx_counter = TransactionCounter::new();

    let port = std::env::var("ALPACA_PORT").unwrap_or_else(|_| "32888".to_string());
    let addr = format!("127.0.0.1:{port}");
    let server = tiny_http::Server::http(&addr).unwrap_or_else(|e| panic!("Failed to bind to {addr}: {e}"));
    eprintln!("ConformU test harness listening on http://{addr}");
    eprintln!("Registered {} devices", registry.configured_devices().len());
    eprintln!("Press Ctrl+C to stop");

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let is_put = request.method() == &tiny_http::Method::Put;
        let server_tx = tx_counter.next();

        let (path, query_str) = match url.split_once('?') {
            Some((p, q)) => (p.to_string(), q.to_string()),
            None => (url.clone(), String::new()),
        };

        // Collect params: query string + PUT form body
        let mut params = parse_query(&query_str);
        if is_put {
            let mut body = String::new();
            request.as_reader().read_to_string(&mut body).ok();
            params.extend(parse_query(&body));
        }

        let client_tx = params
            .get("ClientTransactionID")
            .or_else(|| params.get("clienttransactionid"))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0u32);

        let path_lower = path.to_lowercase();

        let response_body = if path_lower == "/management/apiversions" {
            management::handle_api_versions(client_tx, server_tx)
        } else if path_lower == "/management/v1/description" {
            management::handle_description(client_tx, server_tx)
        } else if path_lower == "/management/v1/configureddevices" {
            management::handle_configured_devices(&registry, client_tx, server_tx)
        } else if let Some((device_type, device_number, method_name)) = parse_device_path(&path) {
            let req = AlpacaRequest {
                device_type,
                device_number,
                method: method_name,
                params,
                is_put,
            };
            let (body, _status) = ascom_alpaca_core::conformu::dispatch::dispatch_request(
                &registry, &req, server_tx,
            );
            body
        } else {
            serde_json::json!({"error": "not found", "path": path}).to_string()
        };

        let resp = tiny_http::Response::from_string(response_body)
            .with_header("Content-Type: application/json".parse::<tiny_http::Header>().unwrap());
        let _ = request.respond(resp);
    }
}
