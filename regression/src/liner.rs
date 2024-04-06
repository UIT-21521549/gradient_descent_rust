fn new_weight(len: usize) -> Vec<f64> {
    let mut weight = Vec::new();
    for _ in 0..len {
        weight.push(0.0);
    }
    weight
}

fn SGD(X: &[f64], y: &[f64], weight: &mut [f64]) {
    // Implement your SGD algorithm here
}
