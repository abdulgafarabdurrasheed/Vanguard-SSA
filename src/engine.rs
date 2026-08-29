use crate::models;

pub fn analyze_signal(x: &models::TelemetryState) {
    println!("Analyzing signal...");
    println!("{} {}", x.satellite_id, x.battery_voltage)
}

pub fn save_to_database(y: &models::TelemetryState) {
    println!("Saving to database...");
    println!("{:?}", y.xyz)
}