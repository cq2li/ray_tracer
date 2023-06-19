pub use std::f64::consts::PI as pi;
pub use std::f64::INFINITY as inf;

pub fn deg_to_rad(degrees: f64) -> f64 {
    return degrees * pi / 180.0;
}
