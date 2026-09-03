use crate::models;
use rand;
use rand::Rng;

pub fn check_collision(x: &models::OrbitalTrajectory) {
    println!("Analyzing signal...");
    println!("{} {} {:?} {:?}", x.satellite_id, x.is_stable, x.predicted_xyz, x.debris_field);

    for debris in &x.debris_field {
        let dist_x = (x.predicted_xyz[0] - debris[0]).powi(2);
        let dist_y = (x.predicted_xyz[1] - debris[1]).powi(2);
        let dist_z = (x.predicted_xyz[2] - debris[2]).powi(2);
        let distance = (dist_x + dist_y + dist_z).sqrt();

        if distance < 100.0 {
            println!("Collision warning! Distance to debris: {:.2} km", distance);
        } else {
            println!("No collision risk. Distance to debris: {:.2} km", distance);
        }
    }
}

pub fn save_to_database(y: &models::TelemetryState) {
    println!("Saving to database...");
    println!("{:?}", y.xyz);
}

pub fn calculate_trajectory(z: &models::TelemetryState, debris: &Vec<[f32; 3]>) -> models::OrbitalTrajectory {
    println!("Calculating trajectory...");

    let name = "ISS (ZARYA)";
    let line1 = "1 25544U 98067A   26243.14365400  .00005331  00000+0  10505-3 0  9995";
    let line2 = "2 25544  51.6314 289.0986 0005054  92.1995 267.9572 15.48946173583375";

    let elements = sgp4::Elements::from_tle(
        Some(name.to_string()),
        line1.as_bytes(),
        line2.as_bytes()
    ).unwrap();

    let constants = sgp4::Constants::from_elements(&elements).unwrap();

    let prediction = constants.propagate(sgp4::MinutesSinceEpoch(z.minutes_since_epoch)).unwrap();

    models::OrbitalTrajectory {
        satellite_id: name.to_string(),
        predicted_xyz: [
            prediction.position[0] as f32,
            prediction.position[1] as f32,
            prediction.position[2] as f32
        ],
        is_stable: true,
        debris_field: debris.clone(),
    }

}

pub fn generate_debris(count: usize) -> Vec<[f32; 3]> {
    let mut rng = rand::thread_rng();
    let mut debris = Vec::new();

    for _ in  0..count{
        let r = rng.gen_range(6500.0..8000.0);
        let theta = rng.gen_range(0.0..std::f32::consts::TAU);
        let phi = rng.gen_range(0.0..std::f32::consts::PI);

        let x = r * phi.sin() * theta.cos();
        let y = r * phi.sin() * theta.sin();
        let z = r * phi.cos();

        debris.push([x, y, z]);
    }
    debris
}