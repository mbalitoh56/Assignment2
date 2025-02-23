use burn::tensor::{backend::NdArray, Tensor};
use burn::nn::{Linear, MSELoss};
use burn::optim::{Adam, Optimizer};
use rand::Rng;
use textplots::{Chart, Plot, Shape};

// Generate synthetic data
fn generate_data(n: usize) -> (Tensor<NdArray, 2>, Tensor<NdArray, 2>) {
    let mut rng = rand::thread_rng();
    let mut x_data = vec![0.0; n];
    let mut y_data = vec![0.0; n];

    for i in 0..n {
        let x: f32 = rng.gen_range(0.0..10.0);
        let noise: f32 = rng.gen_range(-1.0..1.0);
        let y = 2.0 * x + 1.0 + noise;
        x_data[i] = x;
        y_data[i] = y;
    }

    let x_tensor = Tensor::<NdArray, 2>::from_floats(&x_data).reshape([n, 1]);
    let y_tensor = Tensor::<NdArray, 2>::from_floats(&y_data).reshape([n, 1]);

    (x_tensor, y_tensor)
}

// Define a simple linear regression model
struct LinearRegression {
    layer: Linear<NdArray>,
}

impl LinearRegression {
    fn new() -> Self {
        Self {
            layer: Linear::new(1, 1),
        }
    }

    fn forward(&self, x: Tensor<NdArray, 2>) -> Tensor<NdArray, 2> {
        self.layer.forward(x)
    }
}
fn main() {
    println!("Hello, world!");
    let (x_train, y_train) = generate_data(100);
    let model = LinearRegression::new();
    let mut optimizer = Adam::new(model.layer.parameters(), 0.01);

    // Training loop
    for epoch in 0..1000 {
        let y_pred = model.forward(x_train.clone());
        let loss = MSELoss::new().forward(y_pred, y_train.clone());
        optimizer.step(loss.clone());

        if epoch % 100 == 0 {
            println!("Epoch {}: Loss = {:?}", epoch, loss.to_scalar());
        }
    }

    // Evaluation and plotting
    let test_x: Vec<f32> = (0..100).map(|x| x as f32 / 10.0).collect();
    let test_y: Vec<f32> = test_x.iter().map(|x| 2.0 * x + 1.0).collect();
    let plot_data: Vec<(f32, f32)> = test_x.iter().zip(test_y.iter()).map(|(&x, &y)| (x, y)).collect();

    Chart::new(180, 60, 0.0, 10.0)
        .lineplot(&Shape::Lines(&plot_data))
        .display();
}


