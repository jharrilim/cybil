use crate::synapse::Synapse;
use crate::activation::{Activation};

pub struct Neuron {
    inputs: Vec<Synapse>,
    outputs: Vec<Synapse>,
    bias: f32,
    input_total: u32,
    output: f32,

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
    accumulated_derivatives: u32,

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
            input_total: 0,
            output: 0f32,
            output_derivative: 0f32,
            input_derivative: 0f32,
            accumulated_input_derivative: 0f32,
            accumulated_derivatives: 0,
            activation
        }
    }
    pub fn update_output(&mut self, output: f32) {
        self.output = output;
    }
}