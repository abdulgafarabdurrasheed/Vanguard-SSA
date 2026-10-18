mod models;
mod engine;

use std::thread;
use std::time::Duration;
use std::sync::{mpsc, RwLock, Arc};
use axum::{Router, routing::get};
use axum::extract::ws::Utf8Bytes;
use futures::{sink::SinkExt, stream::StreamExt};

#[derive(Clone)]
struct AppState {
    tx_broadcast: tokio::sync::broadcast::Sender<models::OrbitalTrajectory>,
    shared_engine_config: models::SharedEngineConfig,
}
#[tokio::main]
async fn main() {
    let shared_config = Arc::new(RwLock::new(models::EngineConfig {
        time_multiplier: 60.0,
        is_ecef_frame: true,
    }));

    let live_data_future = engine::get_live_data();
    let (live_satellite_data, live_debris_data) = live_data_future.await;
    println!("Live satellite data received: {}", live_satellite_data);
    println!("Live debris data received: {}", live_debris_data);
    let (tx_physics, rx_physics) = mpsc::channel();
    let (tx_broadcast, _rx_dummy) = tokio::sync::broadcast::channel(100);
    let config_clone = Arc::clone(&shared_config);
    thread::spawn(move || {
        let mut simulated_time = chrono::Utc::now().timestamp_millis() as f64 / 1000.0;
        let mut last_real_time = chrono::Utc::now().timestamp_millis() as f64;

        loop {
            let current_real_time = chrono::Utc::now().timestamp_millis() as f64;
            let delta_time_seconds = (current_real_time - last_real_time) / 1000.0;
            last_real_time = current_real_time;

            let multi = config_clone.read().unwrap().time_multiplier;
            let is_ecef = config_clone.read().unwrap().is_ecef_frame;

            simulated_time += delta_time_seconds * multi;

            let telemetry = models::TelemetryState {
                satellite_id: "123456789".to_string(),
                battery_voltage: 12.3,
                xyz: [1.2, 3.4, 5.6],
                universal_timestamp: simulated_time,
                is_ecef_frame: is_ecef,
            };

            let raw_json_string = serde_json::to_string(&telemetry).unwrap();
            thread::sleep(Duration::from_millis(16));
            let back_to_struct: models::TelemetryState = serde_json::from_str(&raw_json_string).unwrap();
            tx_physics.send(back_to_struct).unwrap();
        }
    });
    let tx_thread2 = tx_broadcast.clone();
    thread::spawn(move || {
        println!("Live data received: {}", &live_satellite_data);
        println!("Live debris data received: {}", &live_debris_data);

        let catalog = engine::parse_live_data_to_first_150_strings(&live_satellite_data);
        let debris_catalog = engine::parse_live_data_to_first_150_strings(&live_debris_data);

        for received_data in rx_physics {
            engine::save_to_database(&received_data);
            let mut trajectory = engine::calculate_trajectory(&received_data, &debris_catalog, &catalog);
            trajectory.collision_warning = engine::check_collision(&trajectory);
            tx_thread2.send(trajectory).unwrap();
        }
    });

    let app_state = AppState {
        tx_broadcast: tx_broadcast.clone(),
        shared_engine_config: shared_config.clone(),
    };

    let app: Router = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: axum::extract::ws::WebSocketUpgrade, axum::extract::State(state): axum::extract::State<AppState>) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: axum::extract::ws::WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx_broadcast.subscribe();    tokio::spawn(async move {
        while let Ok(trajectory) = rx.recv().await {
            let json_string = serde_json::to_string(&trajectory).unwrap();
            if sender.send(axum::extract::ws::Message::Text(Utf8Bytes::from(json_string))).await.is_err() {
                println!("WebSocket connection closed. Specific Client disconnected.");
                break;
            }
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        if let axum::extract::ws::Message::Text(text) = msg {
            let new_data: models::EngineConfig = serde_json::from_str(&text).unwrap();
            let mut config = state.shared_engine_config.write().unwrap();
            config.time_multiplier = new_data.time_multiplier;
            config.is_ecef_frame = new_data.is_ecef_frame;
        }
    }
}

//Console JS to stream data from Rust WebSocket server to browser console: 
//const socket = new WebSocket("ws://127.0.0.1:3000/ws");
// socket.onmessage = (event) => console.log("Incoming from Rust:", event.data);