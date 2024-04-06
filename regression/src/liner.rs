fn new_weight(X: &[usize]){
    let len: i32 = X.len();
    let weight = Vec::new();
    for i in 0..len {
        weight.push(0);
    }
    return weight;
}


