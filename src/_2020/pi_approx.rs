//! use the Leibniz formula to approximate pi and stop the iterative process
//! when the absolute value of the difference between our calculation and the
//! Math::PI constant is less than epsilon

fn rnd10(f: f64) -> f64 {
    (f * 1e10).round() / 1e10
}

#[allow(dead_code)]
pub fn iter_pi(epsilon: f64) -> (i32, f64) {
    let mut count = 0;
    let mut pi_approx = 0.0;

    while (pi_approx - std::f64::consts::PI).abs() >= epsilon {
        if count % 2 == 0 {
            pi_approx += 4.0 / (1.0 + (count as f64 * 2.0));
        } else {
            pi_approx -= 4.0 / (1.0 + (count as f64 * 2.0));
        }
        count += 1;
    }
    (count, rnd10(pi_approx))
}
