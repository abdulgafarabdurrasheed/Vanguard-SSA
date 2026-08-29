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

            thread::sleep(Duration::from_secs(1));
            tx_physics.send(telemetry).unwrap();
        }
    });
    thread::spawn(move || {
        for received_data in rx_physics {
            engine::save_to_database(&received_data);
            tx_collision.send(received_data).unwrap();
        }
    });
    for received_data in rx_collision {
        engine::analyze_signal(&received_data);
    }
}
