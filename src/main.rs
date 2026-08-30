mod models;
mod engine;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx_physics, rx_physics) = mpsc::channel();
    let (tx_collision, rx_collision) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let telemetry = models::TelemetryState {
                satellite_id: "123456789".to_string(),
                battery_voltage: 12.3,
                xyz: [1.2, 3.4, 5.6],
            };

            let raw_json_string = serde_json::to_string(&telemetry).unwrap();
            thread::sleep(Duration::from_secs(1));
            let back_to_struct: models::TelemetryState = serde_json::from_str(&raw_json_string).unwrap();
            tx_physics.send(back_to_struct).unwrap();
        }
    });
    thread::spawn(move || {
        for received_data in rx_physics {
            engine::save_to_database(&received_data);
            let trajectory = engine::calculate_trajectory(&received_data);
            tx_collision.send(trajectory).unwrap();
        }
    });
    for received_data in rx_collision {
        engine::check_collision(&received_data);
    }
}
