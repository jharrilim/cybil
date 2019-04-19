use crate::point::Point;
use crate::neuron::Neuron;

pub struct Network {
    neurons: Vec<Neuron>,
    center: Point
}

impl Network {
    pub fn new() -> Network {
        Network {
            neurons: Vec::<Neuron>::new(),
            center: Point::new(0f32, 0f32)
        }
    }

    pub fn add_neuron(&mut self, neuron: Neuron) {
        self.neurons.push(neuron)
    }

    pub fn add_neurons(&mut self, neurons: Vec<Neuron>) {
        self.neurons.extend(neurons)
    }
}
