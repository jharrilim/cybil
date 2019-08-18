use crate::neuron::Neuron;
use crate::activation::{Activation, activation_from, activation_derivative};
use crate::synapse::Synapse;
use petgraph::prelude::{
    NodeIndex,
    Graph,
    Directed,
    EdgeIndex,
    Direction
};
use petgraph::visit::EdgeRef;
use crate::error_function::{ErrorFunc, error_derivative};
use crate::regularization::Regularization;

pub struct Network {
    neuron_graph: Graph<Neuron, Synapse, Directed>,
    nodes_indices: Vec<Vec<NodeIndex<u32>>>,
    edges_indices: Vec<Vec<EdgeIndex<u32>>>,
    epoch: i64,
    activation: Activation,
    output_activation: Activation,
    error_function: ErrorFunc,
    regularization: Regularization
}

impl Network {

    pub fn create(
        shape: Vec<i32>,
        activation: Option<Activation>,
        output_activation: Option<Activation>,
        error_function: Option<ErrorFunc>,
        regularization: Option<Regularization>
    ) -> Network {

        let activation = activation.unwrap_or(Activation::ReLU);
        let output_activation = output_activation.unwrap_or(Activation::ReLU);
        let error_function = error_function.unwrap_or(ErrorFunc::Square);
        let regularization = regularization.unwrap_or(Regularization::L1);
        let mut network = Network {
            neuron_graph: Graph::<Neuron, Synapse>::new(),
            nodes_indices: Vec::<Vec<NodeIndex<u32>>>::new(),
            edges_indices: Vec::<Vec<EdgeIndex<u32>>>::new(),
            epoch: 0i64,
            activation,
            output_activation,
            error_function,
            regularization
        };
        let net_ref = &mut network;
        let amount_of_layers = shape.len();
        // create first layer
        let first_layer = Network::create_layer(shape[0], activation, net_ref);
        net_ref.nodes_indices.push(first_layer);
        // create hidden layers and link neurons with synapses
        for layer_idx in 1..amount_of_layers {
            let layer = Network::create_layer(shape[layer_idx], activation, net_ref);
            for neur_idx in layer.clone() {
                // link this neuron with every neuron from the previous layer
                let mut edge_indices = Vec::<EdgeIndex<u32>>::new();
                for prev_neur_idx in &net_ref.nodes_indices[layer_idx - 1] {
                    let syn = Synapse::new(regularization);
                    let edge = net_ref.neuron_graph.add_edge(*prev_neur_idx, neur_idx, syn);
                    edge_indices.push(edge);
                }
                net_ref.edges_indices.push(edge_indices);
            }
            net_ref.nodes_indices.push(layer);
        }
        network
    }

    fn create_layer(size: i32, activation: Activation, net: &mut Network) -> Vec<NodeIndex<u32>> {
        (0..size)
            .map(|_| {
                let neuron = Neuron::new(activation);
                net.neuron_graph.add_node(neuron)
            })
            .collect::<Vec<NodeIndex<u32>>>()
    }

    fn update_output(&mut self, neuron_index: NodeIndex<u32>) -> f32 {
        let mut input_total = self.neuron_graph[neuron_index].bias.clone();
        for edge in self.neuron_graph.edges_directed(neuron_index, Direction::Incoming) {
            input_total += edge.weight().weight * (self.neuron_graph[edge.source()].output);
        }
        let neuron = &mut self.neuron_graph[neuron_index];
        neuron.input_total = input_total;
        neuron.output = activation_from(neuron.activation)(neuron.input_total);
        println!("Neuron output: {}", neuron.output);
        neuron.output
    }

    pub fn forward_prop(&mut self, inputs: Vec<f32>) -> Result<f32, &str> {
        // Make sure inputs length matches the size of the input vector in the matrix
        if self.nodes_indices[0].len() != inputs.len() {
            return Err("There must be the same amount of inputs as there are nodes in the input layer.");
        }

        // setup input layer
        let mut i = 0;
        for node_idx in self.nodes_indices[0].iter() {
            let mut o = &mut self.neuron_graph[*node_idx];
            o.output = inputs[i];
            i += 1;
        }

        // update layers
        let layer_count = self.nodes_indices.len();
        for i in 1..layer_count {
            let node_indices = &self.nodes_indices[i].clone();
            for node_idx in node_indices {
                self.update_output(*node_idx);
            }
        }
        Ok(self.neuron_graph[self.nodes_indices[layer_count - 1][0]].output)
    }

    pub fn back_prop(&mut self, target: f32) {
        let last_layer_idx = self.nodes_indices.len() - 1;
        let last_layer = &self.nodes_indices[last_layer_idx];
        let mut output_node = &mut self.neuron_graph[last_layer[0]];
        output_node.output_derivative = error_derivative(&self.error_function)(output_node.output, target);


        let mut layer_idx = self.nodes_indices.len() - 1;
        while layer_idx >= 1 {
            // Calculate error derivative w
            for node_index in self.nodes_indices[layer_idx].iter() {
                let mut node = &mut self.neuron_graph[*node_index];
                node.input_derivative =
                    node.output_derivative * activation_derivative(node.activation)(node.input_total);
                node.accumulated_input_derivative += node.input_derivative;
                node.accumulated_derivatives += 1;
            }

            // Calculate error derivative with respect to the weight coming into each node
            for node_index in self.nodes_indices[layer_idx].iter() {
                let mut node = &mut self.neuron_graph[*node_index];
                for edge in self.neuron_graph.edges_directed(*node_index, Direction::Incoming) {
                    let Synapse {
                        is_dead,
                        ..
                    } = edge.weight();
                    if *is_dead {
                        continue
                    }

                    // TODO: finish

                }
            }
            layer_idx -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn create_network() {
        Network::create(
            [2, 4, 2].to_vec(),
            Option::Some(Activation::Tanh),
            Option::from(Activation::Sigmoid),
            Option::from(ErrorFunc::Square),
            Option::Some(Regularization::L1)
        );
    }

    #[test]
    pub fn forward_prop_returns_value_when_given_correct_number_of_inputs() {
        // 4 inputs in the first position
        let shape = [4, 4, 2].to_vec();
        let mut n = Network::create(
            shape,
            Option::from(Activation::ReLU),
            Option::None,
            Option::None,
            Option::Some(Regularization::L2)
        );
        // 4 inputs
        let inputs = [1f32, 5f32, 2601f32, 19238.192f32].to_vec();
        let r = n.forward_prop(inputs);
        assert_ne!(r.unwrap_or(0f32), 0f32);
    }

    #[test]
    pub fn forward_prop_returns_error_when_given_wrong_number_of_inputs() {
        let shape = [1, 2, 3].to_vec();
        let mut n = Network::create(
            shape,
            Option::from(Activation::Linear),
            Option::from(Activation::Sigmoid),
            Option::None,
            Option::from(Regularization::L1)
        );
        let inputs = [1f32, 1000f32, 32f32].to_vec();
        assert!(n.forward_prop(inputs).is_err());
    }
}