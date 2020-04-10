//! given an error threshold `epsilon`, return an iteration count and an
//! approximation of pi using the Leibniz formula, stopping the iterative
//! process when the absolute value of the difference between the approximation
//! and `std::f64::consts::PI` is less than epsilon

/// rounds a floating point number to ten decimal places
fn rnd10(f: f64) -> f64 {
    (f * 1e10).round() / 1e10
}

#[allow(dead_code)]
pub fn iter_pi(epsilon: f64) -> (i32, f64) {
    let mut count = 0;
    let mut pi_approx = 0.0;

    while (pi_approx - std::f64::consts::PI).abs() >= epsilon {
        if count % 2 == 0 {
            pi_approx += 4.0 / f64::from(count).mul_add(1.0, 2.0);
        } else {
            pi_approx -= 4.0 / f64::from(count).mul_add(1.0, 2.0);
        }
        count += 1;
    }
    (count, rnd10(pi_approx))
}
