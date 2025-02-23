
use rand::Rng;

// Define the LinearRegression struct
struct LinearRegression {
    weight: f32,
    bias: f32,
}

impl LinearRegression {
    // Constructor with random initialization
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            weight: rng.gen_range(-0.1..0.1),
            bias: rng.gen_range(-0.1..0.1),
        }
    }

    // Function to make predictions
    fn predict(&self, x: f32) -> f32 {
        (self.weight * x) + self.bias
    }

    // Training function using gradient descent
    fn train(&mut self, x_values: &[f32], y_values: &[f32], epochs: usize, learning_rate: f32) {
        let data_size = x_values.len() as f32;

        for epoch in 0..epochs {
            let mut weight_gradient = 0.0;
            let mut bias_gradient = 0.0;
            let mut total_loss = 0.0;

            for i in 0..x_values.len() {
                let prediction = self.predict(x_values[i]);
                let error = prediction - y_values[i];

                weight_gradient += 2.0 * x_values[i] * error;
                bias_gradient += 2.0 * error;

                total_loss += error.powi(2);
            }

            weight_gradient /= data_size;
            bias_gradient /= data_size;
            total_loss /= data_size;

            self.weight -= learning_rate * weight_gradient;
            self.bias -= learning_rate * bias_gradient;

            if epoch % 10 == 0 {
                println!("Epoch {}: Loss = {:.6}", epoch, total_loss);
            }
        }
    }
}

fn main() {
    let x_train = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y_train = [2.0, 3.9, 6.1, 7.8, 10.2];

    let mut model = LinearRegression::new();

    println!("Starting Training...");
    model.train(&x_train, &y_train, 50, 0.01);

    println!("\nModel Predictions:");
    for x in 0..5 {
        println!("x: {}, predicted y: {:.3}", x, model.predict(x as f32));
    }
}
