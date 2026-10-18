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

pub fn calculate_trajectory(z: &models::TelemetryState, debris_data: &Vec<(sgp4::Elements, sgp4::Constants)>, satellite_data: &Vec<(sgp4::Elements, sgp4::Constants)>) -> models::OrbitalTrajectory {
    println!("Calculating trajectory...");
    let earth_angle = calculate_earth_angle(z.universal_timestamp);
    let mut debris = Vec::new();
    let mut active_satellite = Vec::new();

    for (elements, constants) in debris_data {
        let seconds_since_epoch = z.universal_timestamp - elements.datetime.and_utc().timestamp() as f64;
        let minutes_since_epoch = seconds_since_epoch as f64 / 60.0;
        if let Ok(prediction) = constants.propagate(sgp4::MinutesSinceEpoch(minutes_since_epoch)) {
            if z.is_ecef_frame {
                let x_pos = prediction.position[0];
                let y_pos = prediction.position[1];
                let z_pos = prediction.position[2];

                let rotated_x = x_pos * earth_angle.cos() + y_pos * earth_angle.sin();
                let rotated_y = -x_pos * earth_angle.sin() + y_pos * earth_angle.cos();

                debris.push([rotated_x as f32, rotated_y as f32, z_pos as f32]);
            } else {
                debris.push([
                    prediction.position[0] as f32,
                    prediction.position[1] as f32,
                    prediction.position[2] as f32
                ]);
            }
        }
    }

    for (elements, constants) in satellite_data {
        let seconds_since_epoch = z.universal_timestamp - elements.datetime.and_utc().timestamp() as f64;
        let minutes_since_epoch = seconds_since_epoch as f64 / 60.0;
        if let Ok(prediction) = constants.propagate(sgp4::MinutesSinceEpoch(minutes_since_epoch)) {
            if z.is_ecef_frame {
                let x_pos = prediction.position[0];
                let y_pos = prediction.position[1];
                let z_pos = prediction.position[2];

                let rotated_x = x_pos * earth_angle.cos() + y_pos * earth_angle.sin();
                let rotated_y = -x_pos * earth_angle.sin() + y_pos * earth_angle.cos();

                active_satellite.push([rotated_x as f32, rotated_y as f32, z_pos as f32]);
            } else {
                active_satellite.push([
                    prediction.position[0] as f32,
                    prediction.position[1] as f32,
                    prediction.position[2] as f32
                ]);
            }
        }
    }

    let primary_satellite = satellite_data.first().unwrap();
    let seconds_since_epoch = z.universal_timestamp - primary_satellite.0.datetime.and_utc().timestamp() as f64;
    let minutes_since_epoch = seconds_since_epoch as f64 / 60.0;

    let prediction = primary_satellite
        .1
        .propagate(sgp4::MinutesSinceEpoch(minutes_since_epoch))
        .unwrap();

    let mut final_x = prediction.position[0];
    let mut final_y = prediction.position[1];
    let final_z = prediction.position[2];

    if z.is_ecef_frame {
        let temp_x = final_x;
        let temp_y = final_y;
        final_x = temp_x * earth_angle.cos() + temp_y * earth_angle.sin();
        final_y = -temp_x * earth_angle.sin() + temp_y * earth_angle.cos();
    }

    models::OrbitalTrajectory {
        satellite_id: "unknown_satellite".to_string(),
        predicted_xyz: [
            final_x as f32,
            final_y as f32,
            final_z as f32
        ],
        is_stable: true,
        collision_warning: false,
        debris_field: debris,
        active_satellite,
        universal_timestamp: z.universal_timestamp,
    }
}

pub async fn get_live_data() -> (String, String) {
    let satellite_file_path = "active_satellites.txt";
    let debris_file_path = "debris.txt";
    let mut needs_fetch = true;

    if let (Ok(sat_meta), Ok(deb_meta)) = (fs::metadata(satellite_file_path), fs::metadata(debris_file_path)) {
        if let (Ok(sat_mod), Ok(deb_mod)) = (sat_meta.modified(), deb_meta.modified()) {
            if sat_mod.elapsed().unwrap_or_default().as_secs() < 7200
                && deb_mod.elapsed().unwrap_or_default().as_secs() < 7200 {
                needs_fetch = false;
            }
        }
    }

    if needs_fetch {
        let live_satellite_data = match reqwest::get("https://celestrak.org/NORAD/elements/gp.php?GROUP=active&FORMAT=tle")
            .await {
            Ok(response) => response.text().await.unwrap_or_default(),
            Err(e) => {
                eprintln!("Error fetching satellite data: {}", e);
                fs::read_to_string(satellite_file_path).unwrap_or_default()
            }
        };

        let live_debris_data = match reqwest::get("https://celestrak.org/NORAD/elements/gp.php?GROUP=iridium-33-debris&FORMAT=tle")
            .await {
            Ok(response) => response.text().await.unwrap_or_default(),
            Err(e) => {
                eprintln!("Error fetching debris data: {}", e);
                fs::read_to_string(debris_file_path).unwrap_or_default()
            }
        };

        fs::write(satellite_file_path, &live_satellite_data).unwrap();
        fs::write(debris_file_path, &live_debris_data).unwrap();
        (live_satellite_data, live_debris_data)
    } else {
        (fs::read_to_string(satellite_file_path).unwrap(), fs::read_to_string(debris_file_path).unwrap())
    }
}

pub fn parse_live_data_to_first_150_strings(live_data: &str) -> Vec<(sgp4::Elements, sgp4::Constants)> {
    let lines: Vec<&str> = live_data.lines().collect();
    let mut first_150_strings = Vec::new();
    for line in lines.chunks(3).take(2000) {
        if line.len() == 3 {
            let elements = sgp4::Elements::from_tle(
                Some(line[0].to_string()),
                line[1].as_bytes(),
                line[2].as_bytes()
            ).unwrap();
            first_150_strings.push((elements.clone(), sgp4::Constants::from_elements(&elements).unwrap()));
        }
    }
    first_150_strings
}

pub fn calculate_earth_angle(timestamp: f64) -> f64 {
    let jd = timestamp / 86400.0 + 2440587.5; // Convert Unix timestamp to Julian date
    let t = (jd - 2451545.0) / 36525.0; // Julian centuries since J2000.0
    let gmst = 24110.54841 + (8640184.812866 * t) + (0.093104 * t.powi(2));
    let theta = (gmst + 1.00273790935 * (timestamp % 86400.0)) % 86400.0; // GMST in seconds
    let theta_rad = (theta / 86400.0) * 2.0 * std::f64::consts::PI; // Convert to radians
    theta_rad
}