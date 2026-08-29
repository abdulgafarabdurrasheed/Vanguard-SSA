#[derive(Debug)]
pub struct TelemetryState {
    pub satellite_id: String,
    pub battery_voltage: f32,
    pub xyz: [f32; 3],
}