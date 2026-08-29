use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct TelemetryState {
    satellite_id: String,
    battery_voltage: f32,
    xyz: [f32; 3],
}

fn main() {
    loop {
        let telemetry = TelemetryState {
            satellite_id: "123456789".to_string(),
            battery_voltage: 12.3,
            xyz: [1.2, 3.4, 5.6],
        };

        thread::sleep(Duration::from_secs(1));
        analyze_signal(&telemetry);
        save_to_database(&telemetry);
    }
}

fn analyze_signal(x: &TelemetryState) {
    println!("Analyzing signal...");
    println!("{} {}", x.satellite_id, x.battery_voltage)
}

fn save_to_database(y: &TelemetryState) {
    println!("Saving to database...");
    println!("{:?}", y.xyz)
}
