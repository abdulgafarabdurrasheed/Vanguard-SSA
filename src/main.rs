mod models;
mod engine;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use axum::{Router, routing::get};
use axum::extract::ws::Utf8Bytes;

#[tokio::main]
async fn main() {
    let live_data_future = engine::get_live_data();
    let (live_satellite_data, live_debris_data) = live_data_future.await;
    println!("Live satellite data received: {}", live_satellite_data);
    println!("Live debris data received: {}", live_debris_data);
    let (tx_physics, rx_physics) = mpsc::channel();
    let (tx_broadcast, _rx_dummy) = tokio::sync::broadcast::channel(100);
    thread::spawn(move || {
        let start_time = chrono::Utc::now().timestamp_millis() as f64;
        loop {
            let current_time = chrono::Utc::now().timestamp_millis() as f64;
            let elapsed_time = current_time - start_time;
            let simulated_time = (start_time / 1000.0) + (elapsed_time / 1000.0) * 60.0; // Simulate time passing at 60x speed
            let telemetry = models::TelemetryState {
                satellite_id: "123456789".to_string(),
                battery_voltage: 12.3,
                xyz: [1.2, 3.4, 5.6],
                universal_timestamp: simulated_time,
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

    let app: Router = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(tx_broadcast.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: axum::extract::ws::WebSocketUpgrade, axum::extract::State(tx_broadcast): axum::extract::State<tokio::sync::broadcast::Sender<models::OrbitalTrajectory>>) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx_broadcast))
}

async fn handle_socket(mut socket: axum::extract::ws::WebSocket, tx_broadcast: tokio::sync::broadcast::Sender<models::OrbitalTrajectory>) {
    let mut rx = tx_broadcast.subscribe();
    while let Ok(trajectory) = rx.recv().await {
        let json_string = serde_json::to_string(&trajectory).unwrap();
        if socket.send(axum::extract::ws::Message::Text(Utf8Bytes::from(json_string))).await.is_err() {
            println!("WebSocket connection closed. Specific Client disconnected.");
            break;
        }
    }
}

//Console JS to stream data from Rust WebSocket server to browser console: 
//const socket = new WebSocket("ws://127.0.0.1:3000/ws");
// socket.onmessage = (event) => console.log("Incoming from Rust:", event.data);