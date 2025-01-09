// #![cfg_attr(not(test), no_std)]
pub mod model;
pub mod state;
// pub mod web;

extern crate alloc;

use model::Model;
use state::{build_and_load_model, Backend};

use burn::tensor::Tensor;

#[derive(Default, Clone)]
pub struct M {
    model: Option<Model<Backend>>,
}

impl M {
    pub async fn inference(&mut self, input: &[f64]) -> Result<Vec<f64>, String> {
        if self.model.is_none() {
            self.model = Some(build_and_load_model().await);
        }

        let model = self.model.as_ref().unwrap();

        let device = Default::default();
        // Reshape from the 1D array to 3d tensor [batch, height, width]
        let input = Tensor::<Backend, 1>::from_floats(input, &device).reshape([1, 28, 28]);

        // Normalize input: make between [0,1] and make the mean=0 and std=1
        // values mean=0.1307,std=0.3081 were copied from Pytorch Mist Example
        // https://github.com/pytorch/examples/blob/54f4572509891883a947411fd7239237dd2a39c3/mnist/main.py#L122

        let input = ((input / 255) - 0.1307) / 0.3081;

        // Run the tensor input through the model
        let output: Tensor<Backend, 2> = model.forward(input);

        // Convert the model output into probability distribution using softmax formula
        let output = burn::tensor::activation::softmax(output, 1);

        // Flatten output tensor with [1, 10] shape into boxed slice of [f32]
        let output = output.into_data_async().await;

        Ok(output.iter().collect())
    }
}
