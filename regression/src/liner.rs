fn sigmoid(x: f64) -> f64 {
    let e: f64 = 2.71828;
    1.0 / (1.0 + e.powf(-x))
}

pub struct Model {
    pub x: Vec<Vec<f64>>,
    pub y: Vec<f32>,
    pub weight: Vec<f64>,
    pub loss: f64,
}

impl Model {
    // Khởi tạo model
    pub fn init(&mut self) {
        let len: usize = self.x[0].len();
        self.weight = Self::new_weight(len as i32);
        self.loss = 0.0;
    }

    // Khởi tạo trọng số mới
    fn new_weight(len: i32) -> Vec<f64> {
        vec![0.0; len as usize]
    }

    // Tính loss
    fn _loss(&mut self) {
        let len1 = self.x[0].len();
        let len2 = self.x.len();
        let mut loss = 0.0;
        for i in 0..len2 {
            let mut x_values = 0.0;
            for j in 0..len1 {
                x_values += self.x[i][j] * self.weight[j];
            }
            loss += (self.y[i] - x_values as f32).powi(2);
        }
        loss /= len2 as f32;
        self.loss = sigmoid(loss as f64);
    }

    // Gradient descent
    fn sgd(&mut self, learning_rate: f64, k: usize) -> f64 {
        let len1 = self.x[0].len();
        let len2 = self.x.len();
        let mut gradient = 0.0;
        for i in 0..len2 {
            let mut x_values = 0.0;
            for j in 0..len1 {
                if j == k {
                    continue;
                }
                x_values += self.x[i][j] * self.weight[j];
            }
            gradient += (self.y[i] as f64 - x_values) * self.x[i][k];
        }
        gradient *= learning_rate;
        gradient
    }

    // Cập nhật trọng số
    fn update_weights(&mut self, learning_rate: f64) {
        let len = self.weight.len();
        for i in 0..len {
            self.weight[i] -= self.sgd(learning_rate, i);
        }
        self._loss();
    }

    // Huấn luyện model
    pub fn train(&mut self, epochs: usize, learning_rate: f64) {
        println!("Start training...");
        for _ in 0..epochs {
            self.update_weights(learning_rate);
        }
    }
}

fn main() {
    let mut model = Model {
        x: vec![
            vec![0.5, 6.0, 10.0],
            vec![1.0, 2.0, 3.0],
            vec![8.0, 6.0, 4.0],
            vec![12.0, 11.0, 9.0],
            vec![7.5, 12.0, 13.0],
            vec![13.0, 2.0, 11.0],
        ],
        y: vec![36.5, 13.0, 65.0, 157.0, 162.5, 41.0],
        weight: vec![],
        loss: 0.0,
    };

    model.init();
    model.train(100, 1.0);
}
