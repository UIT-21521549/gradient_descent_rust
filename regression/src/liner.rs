
fn sigmoid(x: f64) -> f64 {
    let e: f64 = 2.71828;
    1.0 / (1.0 + e.powf(-x))
}

pub struct Model {
    pub x: Vec<Vec<f64>>,
    pub y: Vec<f64>, // Changed from Vec<f32> to Vec<f64>
    pub weight: Vec<f64>,
    pub loss: f64,
}

impl Model {
    // Initialize the model
    pub fn new(x: Vec<Vec<f64>>, y: Vec<f64>) -> Self {
        let len = x[0].len();
        let weight = vec![0.0; len];
        let loss = 0.0;
        Self { x, y, weight, loss }
    }

    // Calculate loss
    fn _loss(&mut self) {
        let len1 = self.x[0].len();
        let len2 = self.x.len();
        let mut loss = 0.0;
        for i in 0..len2 {
            let mut x_values = 0.0;
            for j in 0..len1 {
                x_values += self.x[i][j] * self.weight[j];
            }
            loss += (self.y[i] - x_values).powi(2);
        }
        loss /= len2 as f64;
        self.loss = sigmoid(loss);
    }

    // Gradient descent
    // Gradient descent
    fn sgd(&mut self, learning_rate: f64, k: usize, len: usize) -> f64 {
        let len1 = self.x[0].len();
        let len2 = self.x.len();
        let mut gradient = 0.0;
        for i in 0..len2 {
            let mut x_values = 0.0;
            for j in 0..len1 {
                x_values += self.x[i][j] * self.weight[j];
            }
            gradient += (x_values - self.y[i]) * self.x[i][k];
        }; // Average gradient across all data points
        gradient /=len2 as f64;
        gradient
    }


    // Update weights
    fn update_weights(&mut self, learning_rate: f64) {
        self._loss();
        let len = self.weight.len();
        let mut weigh: Vec<f64> = Vec::new();
        for i in 0..len {
            self.weight[i] -= learning_rate*self.sgd(learning_rate, i, len);
        }
        // for i in 0..len {
        //     self.weight[i] -= weigh[i];
        // }
    }

    // Train the model
    pub fn train(&mut self, epochs: usize, learning_rate: f64) {
        println!("Start training...");
        for i in 0..epochs {
            self.update_weights(learning_rate);
            println!("Epoch{}: Loss = {}", i , self.loss);
        }
    }
    pub fn predict(&mut self, x: Vec<f32>) -> f64{
        let len = x.len();
        let mut gradient = 0.0;
        for i in 0..len {
            gradient += x[i] as f64 * self.weight[i];
        };
        gradient +=   self.weight[len];// Average gradient across all data points
        return gradient as f64
    }
}

fn main() {
    let x = vec![
        vec![0.5, 6.0, 10.0],
        vec![1.0, 2.0, 3.0],
        vec![8.0, 6.0, 4.0],
        vec![12.0, 11.0, 9.0],
        vec![7.5, 12.0, 13.0],
        vec![13.0, 2.0, 11.0],
    ];
    let y = vec![36.5, 13.0, 65.0, 157.0, 162.5, 41.0];

    let mut model = Model::new(x, y);
    model.train(100, 1.0);
}
