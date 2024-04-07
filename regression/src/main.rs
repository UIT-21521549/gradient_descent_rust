use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

// Định nghĩa một module mới để chứa định nghĩa của Model
pub mod liner;

// Import Model từ module liner
use crate::liner::Model;

// Định nghĩa hàm để đọc dữ liệu từ file CSV
fn read_csv(file_name: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    
    let mut X: Vec<Vec<f64>> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    let mut first_line_skipped = false;
    for line in reader.lines() {
        let record = line?;
        // Bỏ qua dòng đầu tiên trong file CSV
        if !first_line_skipped {
            first_line_skipped = true;
            continue;
        }
        // Tách các phần từ trong dòng thành mảng
        let parts: Vec<&str> = record.split(',').collect();

        // Chuyển đổi các phần từ sang dạng số thực
        let mut x_values: Vec<f64> = Vec::new();
        for part in &parts[..(parts.len() - 1)] {
            let value: f64 = part.parse().expect("Failed to parse number");
            x_values.push(value);
            // Thêm một phần tử 1.0 ở cuối mỗi vector X để biểu diễn hệ số tự do (intercept)
        }
        x_values.push(1.0); 
        let y_value: f64 = parts[parts.len() - 1].parse().expect("Failed to parse number");

        // Thêm dữ liệu X và y vào các vector tương ứng
        X.push(x_values);
        y.push(y_value);
    }

    // Thêm vector y vào cuối vector X để tạo thành một vector duy nhất chứa toàn bộ dữ liệu
    X.push(y);
    
    // Trả về vector chứa dữ liệu đã đọc được từ file CSV
    Ok(X)
}

fn main() {
    let file_name = "D:/Rust/regression_rust/regression/data/fake_data.csv";
    match read_csv(file_name) {
        Ok(data) => {
            let len = data.len();
            // Chia data thành X và Y
            println!("{:?}",data);
            let X: Vec<Vec<f64>> = data[0..(len - 1)].iter().map(|v| v.clone()).collect();
            let Y: Vec<f64> = data[len - 1].iter().map(|v| *v as f64).collect();
            println!("{:?}",X);
            // Khởi tạo một instance của Model
            let mut model = Model::new(X, Y);

            // Huấn luyện model trong 100 epochs
            let x = vec![0.5, 6.0];
            model.train(2000,0.01);
            let y = model.predict(x);
            println!("Predict: {}", y);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
