pub fn sigmoid(x: f32) -> f32 {
    let e: f64 = 2.71828;
    let k: f64 = 1.0 / (1.0 + e.powf(-x as f64));
    return k as f32;
}
