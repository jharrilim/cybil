use crate::synapse::Synapse;
use crate::activation::{Activation, activation_from};

pub struct Neuron {
    pub inputs: Vec<Synapse>,
    pub outputs: Vec<Synapse>,
    bias: f32,
    input_total: f32,
    pub output: f32,

    /// Error derivative with respect to the node's output.
    output_derivative: f32,

    /// Error derivative with respect to this node's total input.
    input_derivative: f32,


    /// Accumulated error derivative with respect to this node's total input since
    /// the last update. This derivative equals dE/db where b is the node's
    /// bias term.
    accumulated_input_derivative: f32,

    /// Number of accumulated err. derivatives with respect to the total input
    /// since the last update.
    accumulated_derivatives: i32,

    /// A function that accepts an input, in this case the sum of the total input, and returns
    /// an output.
    activation: Activation
}

impl Neuron {
    pub fn new(activation: Activation) -> Neuron {
        Neuron {
            inputs: Vec::new(),
            outputs: Vec::new(),
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

    pub fn update_output(&mut self) -> f32 {
        self.input_total = self.bias;
        for &input in self.inputs {
            self.input_total += input.weight * input.source.output;
        }
        self.output = activation_from(self.activation)(self.input_total);
        self.output
    }
}

