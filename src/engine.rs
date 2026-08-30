use crate::models;

pub fn check_collision(x: &models::OrbitalTrajectory) {
    println!("Analyzing signal...");
    println!("{} {}", x.satellite_id, x.is_stable);
}

pub fn save_to_database(y: &models::TelemetryState) {
    println!("Saving to database...");
    println!("{:?}", y.xyz);
}

pub fn calculate_trajectory(z: &models::TelemetryState) -> models::OrbitalTrajectory {
    println!("Calculating trajectory...");
    models::OrbitalTrajectory {
        satellite_id: z.satellite_id.clone(),
        predicted_xyz: z.xyz,
        is_stable: true,
    }
}