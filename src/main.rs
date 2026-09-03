mod models;
mod engine;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use axum::{Router, routing::get};
use axum::extract::ws::Utf8Bytes;

#[tokio::main]
async fn main() {
    let (tx_physics, rx_physics) = mpsc::channel();
    let (tx_broadcast, _rx_dummy) = tokio::sync::broadcast::channel(100);
    thread::spawn(move || {
        let mut current_time = 0.0;
        let time_step = 0.144; //for 1sec = 9 minutes or 0.000266 for 1 sec = 1 seconds or 0.024 for 1 sec = 90 seconds
        loop {
            current_time += time_step;
            let telemetry = models::TelemetryState {
                satellite_id: "123456789".to_string(),
                battery_voltage: 12.3,
                xyz: [1.2, 3.4, 5.6],
                minutes_since_epoch: current_time,
            };

            let raw_json_string = serde_json::to_string(&telemetry).unwrap();
            thread::sleep(Duration::from_millis(16));
            let back_to_struct: models::TelemetryState = serde_json::from_str(&raw_json_string).unwrap();
            tx_physics.send(back_to_struct).unwrap();
        }
    });
    let mut rx_radar = tx_broadcast.subscribe();
    let tx_thread2 = tx_broadcast.clone();
    thread::spawn(move || {
        let debris = engine::generate_debris(10);
        for received_data in rx_physics {
            engine::save_to_database(&received_data);
            let trajectory = engine::calculate_trajectory(&received_data, &debris);
            tx_thread2.send(trajectory).unwrap();
        }
    });
    tokio::spawn(async move {
        while let Ok(received_data) = rx_radar.recv().await {
            engine::check_collision(&received_data);
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