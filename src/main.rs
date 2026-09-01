mod models;
mod engine;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx_physics, rx_physics) = mpsc::channel();
    let (tx_broadcast, _rx_dummy) = tokio::sync::broadcast::channel(100);
    thread::spawn(move || {
        let mut current_time = 0.0;
        loop {
            current_time += 1.5;
            let telemetry = models::TelemetryState {
                satellite_id: "123456789".to_string(),
                battery_voltage: 12.3,
                xyz: [1.2, 3.4, 5.6],
                minutes_since_epoch: current_time,
            };

            let raw_json_string = serde_json::to_string(&telemetry).unwrap();
            thread::sleep(Duration::from_secs(1));
            let back_to_struct: models::TelemetryState = serde_json::from_str(&raw_json_string).unwrap();
            tx_physics.send(back_to_struct).unwrap();
        }
    });
    let mut rx_radar = tx_broadcast.subscribe();
    let tx_thread2 = tx_broadcast.clone();
    thread::spawn(move || {
        for received_data in rx_physics {
            engine::save_to_database(&received_data);
            let trajectory = engine::calculate_trajectory(&received_data);
            tx_thread2.send(trajectory).unwrap();
        }
    });
    while let Ok(received_data) = rx_radar.recv().await {
        engine::check_collision(&received_data);
    }
}