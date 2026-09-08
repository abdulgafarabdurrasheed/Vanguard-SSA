use crate::models;
use std::fs;

pub fn check_collision(x: &models::OrbitalTrajectory) -> bool {
    let mut collision_warning = false;
    println!("Analyzing signal...");
    println!("{} {} {:?} {:?}", x.satellite_id, x.is_stable, x.predicted_xyz, x.debris_field);

    for debris in &x.debris_field {
        let dist_x = (x.predicted_xyz[0] - debris[0]).powi(2);
        let dist_y = (x.predicted_xyz[1] - debris[1]).powi(2);
        let dist_z = (x.predicted_xyz[2] - debris[2]).powi(2);
        let distance = (dist_x + dist_y + dist_z).sqrt();

        if distance < 1000.0 {
            println!("Collision warning! Distance to debris: {:.2} km", distance);
            collision_warning = true;
            break;
        }
    }
    collision_warning
}

pub fn save_to_database(y: &models::TelemetryState) {
    println!("Saving to database...");
    println!("{:?}", y.xyz);
}

pub fn calculate_trajectory(z: &models::TelemetryState, live_data: &Vec<sgp4::Constants>) -> models::OrbitalTrajectory {
    println!("Calculating trajectory...");
    let mut debris = Vec::new();

    for satellite in live_data {
        let prediction = satellite.propagate(sgp4::MinutesSinceEpoch(z.minutes_since_epoch)).unwrap();
        debris.push([
            prediction.position[0] as f32,
            prediction.position[1] as f32,
            prediction.position[2] as f32
        ]);
    }
    let primary_satellite = live_data.first().unwrap();
    let prediction = primary_satellite
        .propagate(sgp4::MinutesSinceEpoch(z.minutes_since_epoch))
        .unwrap();

    models::OrbitalTrajectory {
        satellite_id: "unknown_satellite".to_string(),
        predicted_xyz: [
            prediction.position[0] as f32,
            prediction.position[1] as f32,
            prediction.position[2] as f32
        ],
        is_stable: true,
        collision_warning: false,
        debris_field: debris,
    }

}

pub async fn get_live_data() -> String {
    let file_path = "active_satellites.txt";
    let mut needs_fetch = true;

    if let Ok(metadata) = fs::metadata(file_path) {
        if let Ok(modified) = metadata.modified() {
            if modified.elapsed().unwrap_or_default().as_secs() < 7200 {
                needs_fetch = false;
            }
        }
    }

    if needs_fetch {
        let live_data = reqwest::get("https://celestrak.org/NORAD/elements/gp.php?GROUP=active&FORMAT=tle")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        fs::write(file_path, &live_data).unwrap();
        live_data
    } else {
        fs::read_to_string(file_path).unwrap()
    }
}

pub fn parse_live_data_to_first_150_strings(live_data: &str) -> Vec<sgp4::Constants> {
    let lines: Vec<&str> = live_data.lines().collect();
    let mut first_150_strings = Vec::new();
    for line in lines.chunks(3).take(2000) {
        if line.len() == 3 {
            let elements = sgp4::Elements::from_tle(
                Some(line[0].to_string()),
                line[1].as_bytes(),
                line[2].as_bytes()
            ).unwrap();
            first_150_strings.push(sgp4::Constants::from_elements(&elements).unwrap())
        }
    }
    first_150_strings
}