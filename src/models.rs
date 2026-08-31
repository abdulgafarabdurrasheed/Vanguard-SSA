use serde::{Deserialize, Serialize};
use sgp4::MinutesSinceEpoch;

#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetryState {
    pub satellite_id: String,
    pub battery_voltage: f32,
    pub xyz: [f32; 3],
    pub minutes_since_epoch: f64,
}

pub struct OrbitalTrajectory {
    pub satellite_id: String,
    pub predicted_xyz: [f32; 3],
    pub is_stable: bool,
}