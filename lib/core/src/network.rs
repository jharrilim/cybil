use nalgebra::Vector2;

use crate::neuron::Neuron;
use nalgebra::base::Matrix;
use nalgebra::base::dimension::Dynamic;

pub struct Network {
    neurons: Vec<Neuron>,
    center: Vector2<f32>,
    hidden_layer_matrix: Vec<Vec<Neuron>>
}

impl Network {
    pub fn new() -> Network {
        Network {
            neurons: Vec::<Neuron>::new(),
            center: Vector2::new(0f32, 0f32),
            hidden_layer_matrix: Vec::new()
        }
    }

    pub fn add_neuron(&mut self, neuron: Neuron) {
        self.neurons.push(neuron)
    }

    pub fn add_neurons(&mut self, neurons: Vec<Neuron>) {
        self.neurons.extend(neurons)
    }

}
