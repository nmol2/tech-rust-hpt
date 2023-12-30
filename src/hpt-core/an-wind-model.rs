// beaufort ranges between 0 to 12 inclusive
fn beaufort_to_wind(beaufort: f32)->f32{
    let wind_range:[f32;13] = [0.1_f32, 0.9_f32, 2.45_f32, 4.4_f32, 6.7_f32,
    9.35_f32, 12.3_f32, 15.5_f32, 18.9_f32, 22.1_f32, 26.5_f32, 30.5_f32, 35_f32];
    wind_range[beaufort.ceil().clamp(0_f32,12_f32) as usize]
} 


// Inputs: Angles in degrees, speeds in m/s!
// WindSpeed is true, absolute wind speed
// same for WindDirection.
// Output in degrees!
fn get_apparent_wind_direction(vessel_speed: f64, cog: f64, wind_speed: f64, wind_direction: f64) -> f64 {
    let wind_direction_rad = wind_direction.to_radians();
    let vessel_course = cog.to_radians();
    
    let x = wind_direction_rad - vessel_course;
    let cos_x = x.cos();
    let sin_x = x.sin();
    let speed_term = vessel_speed + wind_speed * cos_x;
    
    let apparent_wind_direction_rad = if speed_term > 0.0 {
        (wind_speed * sin_x / speed_term).atan()
    } else if speed_term < 0.0 {
        (wind_speed * sin_x / speed_term).atan() + PI
    } else {
        0.0
    };
    
    apparent_wind_direction_rad.to_degrees()
}


fn get_apparent_wind_speed(vessel_speed: f64, cog: f64, wind_speed: f64, wind_direction: f64) -> f64 {
    let wind_direction_rad = wind_direction.to_radians();
    let vessel_course = cog.to_radians();

    ((vessel_speed.powi(2) + wind_speed.powi(2) + 2.0 * vessel_speed * wind_speed * (wind_direction_rad - vessel_course).cos()).sqrt()) as f64
}

// use std::cmp;

fn get_cx(wind_direction: f64) -> f64 {
    let mut first_cx = 0.0;
    let mut second_cx = 0.0;
    let mut first_degree = 0.0;
    let mut second_degree = 0.0;
    let mut cx_value = 0.0;

    // Convert wind direction into [0,180]-interval
    let wind_direction = if wind_direction > 180.0 {
        360.0 - wind_direction
    } else if wind_direction < 0.0 {
        // f64::from(cmp::max(0, 
            wind_direction.abs()
        // ))
    } else {
        wind_direction
    };

    // Get the CXValue
    for j in 0..wind_cx_values_array.len() {
        first_degree = wind_cx_values_array[j][0];
        first_cx = wind_cx_values_array[j][1];
        if j != wind_cx_values_array.len() - 1 {
            second_degree = wind_cx_values_array[j + 1][0];
            second_cx = wind_cx_values_array[j + 1][1];
        }
        if wind_direction >= first_degree && wind_direction <= second_degree {
            cx_value = first_cx + (second_cx - first_cx) / (second_degree - first_degree) * (wind_direction - first_degree);
            break;
        }
    }
    
    cx_value
}


// use decimal::d128; // Add the decimal crate to your dependencies

// fn beaufort_to_wind(beaufort: f64) -> f64 {
//     // Add your implementation for beaufort_to_wind
// }

// fn get_apparent_wind_direction(vessel_speed: f64, cog: f64, wind_speed: f64, wind_direction: f64) -> f64 {
//     // Add your implementation for get_apparent_wind_direction
// }

// fn get_apparent_wind_speed(vessel_speed: f64, cog: f64, wind_speed: f64, wind_direction: f64) -> f64 {
//     // Add your implementation for get_apparent_wind_speed
// }

// fn get_cx(wind_direction: f64) -> d128 {
//     // Add your implementation for get_cx
// }

// fn load_wind_cx_values(operation_mode: &str, wind_model_name: &str) {
//     // Add your implementation for loading wind_cx_values
// }

fn get_wind_resistance(
    operation_mode: &str, 
    wind_model_name: &str, 
    wind_area: f64, 
    wind_force: f64, 
    true_wind_direction: f64, 
    observed_vessel_speed: f64, 
    rho_air: f64, 
    course_over_ground: f64, 
    relative_wind_given: bool, 
    rel_wind_speed_in_ms: f64, 
    rel_wind_dir_degrees: f64, 
    true_winf_force_is_wind_speed_ms: bool
) -> f64 {
    let mut true_wind_speed = 0.0;
    let mut relative_wind_speed = 0.0;
    let mut relative_wind_direction = 0.0;
    // let mut cx_wind_zero = 0.0;
    // let mut cx_wind = 0.0;
    // let mut wind_resistance_raw = 0.0;
    // let mut wind_resistance_zero = 0.0;

    // Get the true wind speed based on Beaufort scale (Douglas)
    if !relative_wind_given {
        if !true_winf_force_is_wind_speed_ms {
            true_wind_speed = beaufort_to_wind(wind_force);
        } else {
            true_wind_speed = wind_force;
        }

        // Get the apparent wind direction
        relative_wind_direction = get_apparent_wind_direction(
            observed_vessel_speed / 1.94384, 
            course_over_ground, 
            true_wind_speed, 
            true_wind_direction
        );

        relative_wind_speed = get_apparent_wind_speed(
            observed_vessel_speed / 1.94384, 
            course_over_ground, 
            true_wind_speed, 
            true_wind_direction
        );
    } else {
        relative_wind_speed = rel_wind_speed_in_ms;
        relative_wind_direction = rel_wind_dir_degrees;
    }

    // LOAD the wind Model
    // if wind_cx_values_loaded == False or operation_mode_cx_array != operation_mode:
    load_wind_cx_values(operation_mode, wind_model_name);

    // Directions and Speeds
    // GetCX values
    let cx_wind_zero: f64 = get_cx(0.0);
    let cx_wind: f64 = get_cx(relative_wind_direction);

    let wind_area_term: f64 = (1 / 2 * rho_air) * wind_area / 1000.0;
    let wind_resistance_raw_wo_wat = relative_wind_speed.powi(2) * cx_wind;
    let wind_resistance_zero_wo_wat = (observed_vessel_speed / 1.94384).powi(2) * cx_wind_zero;
    wind_area_term * (wind_resistance_raw - wind_resistance_zero)
    // Calculate the wind resistance
    // let wind_resistance_raw = (1 / 2 * rho_air) * wind_area * (relative_wind_speed.powi(2)) * cx_wind / 1000.0;
    // let wind_resistance_zero = (1 / 2 * rho_air) * wind_area * (observed_vessel_speed / 1.94384).powi(2) * cx_wind_zero / 1000.0;

    // careful with the sign, kn
    // wind_resistance_raw - wind_resistance_zero

}


use std::collections::HashMap;
use decimal::d128; // Add the decimal crate to your dependencies

fn get_models(model_name: &str) -> HashMap<&'static str, Vec<f64>> {
    // Add your implementation for retrieving wind models
    // This function should return a HashMap with wind model data.
}

static mut WIND_CX_VALUES_ARRAY: Option<Vec<[f64; 2]>> = None;
static mut OPERATION_MODE_CX_ARRAY: Option<String> = None;

fn load_wind_cx_values(operation_mode: &str, wind_model_name: &str) {
    unsafe {
        let mut wind_cx_values_array = WIND_CX_VALUES_ARRAY.take().unwrap_or_else(|| Vec::new());
        let mut operation_mode_cx_array = OPERATION_MODE_CX_ARRAY.take().unwrap_or_else(|| String::new());

        if wind_cx_values_array.is_empty() || operation_mode_cx_array != operation_mode {
            let operation_mode_map = hashmap!{
                "Ballast" => 3,
                "Design" => 2,
                "Scatling" => 1,
                "Laden" => 1,
            };
            let wind_model_name_map = hashmap!{
                "Blendermann 300" => "blendermann300",
                "Tanker ISO19030" => "tankerISO19030",
                "HandySize Bulk Carrier" => "bulkCarrier",
            };

            let wind_model = get_models(wind_model_name_map[wind_model_name]);
            for x in wind_model {
                wind_cx_values_array.push([x[0], x[operation_mode_map[operation_mode]]]);
            }

            WIND_CX_VALUES_ARRAY = Some(wind_cx_values_array);
            OPERATION_MODE_CX_ARRAY = Some(operation_mode.to_string());
        }
    }
}





#[derive(Debug)]
struct WindModel {
    wind_min_speed: f64,
    beaufort: i32,
}

fn get_wind_model() -> Vec<WindModel> {
    // Assuming this function returns the wind model data.
    // You should implement it based on your specific data source.
    // Replace this with your actual implementation.
    vec![
        WindModel { wind_min_speed: 2.0, beaufort: 1 },
        WindModel { wind_min_speed: 6.0, beaufort: 2 },
        WindModel { wind_min_speed: 12.0, beaufort: 3 },
        // Add more data as needed
    ]
}

fn wind_to_beaufort(wind_speed: f64) -> i32 {
    if wind_speed == 0.0 {
        return 0;
    }
    
    let wind_model = get_wind_model();
    let mut respective_wind_speed = 0.0;
    for x in &wind_model {
        respective_wind_speed = x.wind_min_speed;
        if wind_speed < respective_wind_speed {
            return x.beaufort;
        }
    }
    if wind_speed > respective_wind_speed {
        return wind_model.last().unwrap().beaufort;
    }
    0
}


