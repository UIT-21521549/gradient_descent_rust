use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};


// read csv data
/*file_name is path of file, X is take  as input vector and Y is output vector*/
fn read_csv(file_name: &str) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    
    let mut X: Vec<Vec<f64>> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    let mut first_line_skipped = false;
    for line in reader.lines() {
        let record = line?;
        //pass header of csv
        if !first_line_skipped {
            first_line_skipped = true;
            continue;
        }
        // split in string
        let parts: Vec<&str> = record.split(',').collect();
        println!("{:?}", parts);
        // change to f64
        let mut x_values: Vec<f64> = Vec::new();
        for part in &parts[..(parts.len() - 1)] {
            let value: f64 = part.parse().expect("Failed to parse number");
            x_values.push(value);
        }
        let y_value: f64 = parts[parts.len() - 1].parse().expect("Failed to parse number");;

        //add data
        X.push(x_values);
        y.push(y_value);

        println!("{:?}", record);
    }

    println!("X: {:?}", X);
    println!("Y: {:?}", y);
    X.push(y);
    Ok((X))
}

fn main() {
    let file_name = "D:/Rust/regression_rust/regression/data/fake_data.csv";
    
}
