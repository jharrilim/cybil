use crate::neuron::Neuron;
use crate::activation::{Activation};
use crate::synapse::Synapse;
use petgraph::Graph;
pub struct Network {
    neuron_matrix: Vec<Vec<Neuron>>,
    epoch: i64,
    activation: Activation,
    output_activation: Activation
}

impl Network {

    pub fn create(
        shape: Vec<i32>,
        activation: Option<Activation>,
        output_activation: Option<Activation>) -> Network {
        let activation = activation.unwrap_or(Activation::ReLU);
        let output_activation = output_activation.unwrap_or(Activation::ReLU);
        let mut network = Network {
            neuron_matrix: Vec::new(),
            epoch: 0i64,
            activation,
            output_activation
        };
        let amount_of_layers = shape.len();
        let hidden_layers_count = amount_of_layers - 1;
        // create first layer
        network.neuron_matrix.push(Network::create_layer(shape[0], activation));
        // create hidden layers and link neurons with synapses
        for layer_idx in 1..hidden_layers_count {
            let layer = Network::create_layer(shape[layer_idx], activation);
            for mut neuron in layer {
                // link this neuron with every neuron from the previous layer
                for mut prev_neuron in network.neuron_matrix[layer_idx - 1] {
                    let syn = Synapse::new(neuron, prev_neuron);
                    neuron.inputs.push(syn);
                    prev_neuron.outputs.push(syn);
                }

            }
            network.neuron_matrix.push(layer);
        }

        network
    }

    fn create_layer(size: i32, activation: Activation) -> Vec<Neuron> {
        (0..size)
            .map(|_| Neuron::new(activation))
            .collect::<Vec<Neuron>>()
    }

    pub fn add_hidden_layer(&mut self, size: i32) {
        if size == 0 {
            self.neuron_matrix.push(Vec::new());
        } else {
            let layer: Vec<Neuron> = Network::create_layer(size, self.activation);
            self.neuron_matrix.push(layer);
        }
    }

    pub fn remove_hidden_layer_at(&mut self, index: usize) {
        self.neuron_matrix.remove(index);
    }

    pub fn forward_prop(&mut self, inputs: Vec<f32>) {
        // setup input layer
        self.neuron_matrix[0] = (0..inputs.len())
            .map(|i| {
                let mut n = Neuron::new(self.activation);
                n.output = inputs[i];
                n
            }
        ).collect::<Vec<Neuron>>();

        for i in 1..self.neuron_matrix.len() {
            let layer = self.neuron_matrix[i];
            for mut node in layer {
                node.update_output();
            }
        }
    }
}

