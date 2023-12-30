use chrono::{Datelike, TimeZone, Timelike, Utc};
// use decimal::d128; // Add the decimal crate to your dependencies

fn date_to_float(datetime_val: Option<chrono::DateTime<Utc>>) -> Option<f64> {
    match datetime_val {
        Some(val) => {
            let date_diff = val.signed_duration_since(Utc.ymd(1900, 1, 1).and_hms(0, 0, 0));
            let floated_days = (date_diff.num_seconds() + 1) as f64 / 86400.0 + date_diff.num_days() as f64 + 1.0;
            Some(floated_days)
        }
        None => None,
    }
}


fn ntz<T>(val: Option<T>) -> T {
    val.unwrap_or(0)
}


fn custom_sqrt(val: f64) -> f64 {
    if val < 0.0 {
        -1.0 * val.abs().sqrt()
    } else {
        val.sqrt()
    }
}

use chrono::{Datelike, Timelike, TimeZone, Utc};
use decimal::d128; // Add the decimal crate to your dependencies

fn f2dt(float_val: Option<f64>) -> Option<chrono::DateTime<Utc>> {
    match float_val {
        Some(val) => {
            let days = float_val.trunc() as i64;
            let remainder = float_val.fract();
            let hours_float = remainder * 24.0;
            let hours = hours_float.trunc() as u32;
            let remainder = hours_float.fract();
            let minutes_float = remainder * 60.0;
            let minutes = minutes_float.trunc() as u32;
            let remainder = minutes_float.fract();
            let seconds_float = remainder * 60.0;
            let seconds = seconds_float.trunc() as u32;
            Some(Utc.ymd(1900, 1, 1).and_hms(hours, minutes, seconds))
        }
        None => None,
    }
}
