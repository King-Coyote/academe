

pub fn round_to(val: f32, precision: u32) -> f32 {
    let precision = 10.0_f32.powi(precision as i32);
    (val * precision).round() / precision
}