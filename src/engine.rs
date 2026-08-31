use crate::models;

pub fn check_collision(x: &models::OrbitalTrajectory) {
    println!("Analyzing signal...");
    println!("{} {} {:?}", x.satellite_id, x.is_stable, x.predicted_xyz);

    let debris_xyz: [[f32; 3]; 3] = [
        [1000.0, 2000.0, 3000.0],
        [3878.0, -4941.0, 2588.0],
        [-5000.0, 1000.0, -1000.0],
    ];

    for debris in debris_xyz {
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

pub fn calculate_trajectory(z: &models::TelemetryState) -> models::OrbitalTrajectory {
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
    }
}