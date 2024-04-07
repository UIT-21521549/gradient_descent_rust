use crate::sigmoid::sigmoid; 

fn new_weight(len: usize) -> Vec<f64> {
    let mut weight = Vec::new();
    for _ in 0..=len {
        weight.push(0.0);
    }
    weight
}
//caculate loss of 2 vector
fn loss(X: &[[f64]], y: &[f64], weight: &mut [f64]) -> f32{
    let len1 = x[0].len();
    let len2 = x.len();
    let mut loss = 0.0;
    let mut x_values = 0.0;
    for i in 0..len1{
        for j in 0..len2{
            x_values +=  X[i][j]*weight[j];
        }
        loss += powf(y[i]  - x_values + weight[len2] , 2);
    }
    loss /= len1 as f32;
    loss = sigmoid(loss);
    loss
}

//caculate gadient
fn SGD(X: &[[f64]], y: &[f64], weight: &mut [f64], k: i16) -> f64 {
    let len1 = x[0].len();
    let len2 = x.len();
    let mut loss = 0.0;
    let mut x_values = 0.0;
    for i in 0..len1{
        for j in 0..len2{
            if j == k {
                continue;
            }
            x_values += X[i][j]*weight[j];
        }
        loss += (x_values + weight[k] + weight[len2] - y[i]) * X[i][k];
    }
    loss = loss * 2 * loss / len1;
    loss
}

fn  update_weights(x: &[f64], y: &[f64], weights: &mut [f64], losses: &[f64]) {

}