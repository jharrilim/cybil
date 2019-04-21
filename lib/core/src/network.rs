use crate::neuron::Neuron;
use crate::activation::Activation;

pub struct Network {
    neurons: Vec<Neuron>,
    hidden_layer_matrix: Vec<Vec<Neuron>>,
    epoch: u64,
    activation: Activation
}

impl Network {
    pub fn new() -> Network {
        Network {
            neurons: Vec::<Neuron>::new(),
            hidden_layer_matrix: Vec::new(),
            epoch: 0u64,
            activation: Activation::ReLU
        }
    }

    pub fn add_neuron(&mut self, neuron: Neuron) {
        self.neurons.push(neuron)
    }

    pub fn add_neurons(&mut self, neurons: Vec<Neuron>) {
        self.neurons.extend(neurons)
    }

    pub fn add_hidden_layer(&mut self, size: u32, activation: Option<Activation>) {
        if size == 0 {
            self.hidden_layer_matrix.push(Vec::new());
        } else {
            let layer: Vec<Neuron> = (0..size)
                .map(|_| Neuron::new(
                    activation
                        .unwrap_or(Activation::ReLU)
                )).collect::<Vec<Neuron>>();
            self.hidden_layer_matrix.push(layer);
        }
    }



    pub fn remove_hidden_layer_at(&mut self, index: usize) {
        self.hidden_layer_matrix.remove(index);
    }

    pub fn forward_prop(&mut self, inputs: Vec<f32>) {
        self.neurons.clear();
        self.neurons = (0..inputs.len())
            .map(|i| {
                let mut n = Neuron::new(self.activation);
                n.update_output(inputs[i]);
                n
            }
        ).collect::<Vec<Neuron>>();
    }
}

