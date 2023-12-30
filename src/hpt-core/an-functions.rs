use decimal::d128; // Add the decimal crate to your dependencies

fn calc_trendline(x_val: Vec<f64>, y_val: Vec<f64>, return_incline: bool, return_magnitude: bool) -> f64 {
    let mut sum_calc_a = 0.0;
    let mut sum_calc_b_x = 0.0;
    let mut sum_calc_b_y = 0.0;
    let mut sum_calc_c = 0.0;
    let mut trendline_m = 0.0;
    let mut trendline_q = 0.0;

    let mut n = 0;
    let mut i = 1;

    while i < x_val.len() {
        n += 1;
        let x_val_i = date_to_float(x_val[i]);
        sum_calc_a += x_val_i * y_val[i];
        sum_calc_b_x += x_val_i;
        sum_calc_b_y += y_val[i];
        sum_calc_c += x_val_i.powi(2);
        i += 1;
    }

    if sum_calc_b_y == 0.0 {
        trendline_m = 0.0;
        trendline_q = 0.0;
        return 0.0;
    } else {
        let a = n * sum_calc_a;
        let b = sum_calc_b_x * sum_calc_b_y;
        let c = n * sum_calc_c;
        let d = sum_calc_b_x * sum_calc_b_x;
        let e = sum_calc_b_y;

        if c - d == 0.0 || n == 0 {
            trendline_m = 0.0;
            trendline_q = 0.0;
            return 0.0;
        } else {
            if !(c - d == 0.0) {
                trendline_m = (a - b) / (c - d);
                let f = trendline_m * sum_calc_b_x;
                trendline_q = (e - f) / n;
            }
        }
    }

    // return value
    if return_incline {
        trendline_m
    } else if return_magnitude {
        trendline_q
    } else {
        0.0
    }
}
