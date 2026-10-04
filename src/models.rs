use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetryState {
    pub satellite_id: String,
    pub battery_voltage: f32,
    pub xyz: [f32; 3],
    pub universal_timestamp: f64
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrbitalTrajectory {
    pub satellite_id: String,
    pub predicted_xyz: [f32; 3],
    pub is_stable: bool,
    pub debris_field: Vec<[f32; 3]>,
    pub collision_warning: bool,
    pub active_satellite: Vec<[f32; 3]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineConfig {
    pub time_multiplier: f64,
    pub is_ecef_frame: bool,
}

pub type SharedEngineConfig = Arc<RwLock<EngineConfig>>;
