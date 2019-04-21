use crate::activation::{Activation};

pub struct Neuron {
    pub bias: f32,
    pub input_total: f32,
    pub output: f32,

    /// Error derivative with respect to the node's output.
    pub output_derivative: f32,

    /// Error derivative with respect to this node's total input.
    pub input_derivative: f32,


    /// Accumulated error derivative with respect to this node's total input since
    /// the last update. This derivative equals dE/db where b is the node's
    /// bias term.
    pub accumulated_input_derivative: f32,

    /// Number of accumulated err. derivatives with respect to the total input
    /// since the last update.
    pub accumulated_derivatives: i32,

    /// A function that accepts an input, in this case the sum of the total input, and returns
    /// an output.
    pub activation: Activation
}

impl Neuron {
    pub fn new(activation: Activation) -> Neuron {
        Neuron {
            bias: 0.1f32,
            input_total: 0f32,
            output: 0f32,
            output_derivative: 0f32,
            input_derivative: 0f32,
            accumulated_input_derivative: 0f32,
            accumulated_derivatives: 0,
            activation
        }
    }
}

