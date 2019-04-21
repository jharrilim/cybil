use crate::neuron::Neuron;
use crate::activation::{Activation, activation_from};
use crate::synapse::Synapse;
use petgraph::prelude::{
    NodeIndex,
    Graph,
    Directed,
    EdgeIndex,
    Direction
};
use petgraph::visit::EdgeRef;

pub struct Network {
    neuron_graph: Graph<Neuron, Synapse, Directed>,
    nodes_indices: Vec<Vec<NodeIndex<u32>>>,
    edges_indices: Vec<Vec<EdgeIndex<u32>>>,
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
            neuron_graph: Graph::<Neuron, Synapse>::new(),
            nodes_indices: Vec::<Vec<NodeIndex<u32>>>::new(),
            edges_indices: Vec::<Vec<EdgeIndex<u32>>>::new(),
            epoch: 0i64,
            activation,
            output_activation
        };
        let net_ref = &mut network;
        let amount_of_layers = shape.len();
        // create first layer
        net_ref.nodes_indices.push(Network::create_layer(shape[0], activation, net_ref));
        // create hidden layers and link neurons with synapses
        for layer_idx in 1..amount_of_layers {
            let layer = Network::create_layer(shape[layer_idx], activation, net_ref);
            net_ref.nodes_indices.push(layer);
            for mut neur_idx in layer {
                // link this neuron with every neuron from the previous layer
                let mut edge_indices = Vec::<EdgeIndex<u32>>::new();
                for mut prev_neur_idx in network.nodes_indices[layer_idx - 1] {
                    let syn = Synapse::new();
                    let edge = network.neuron_graph.add_edge(prev_neur_idx, neur_idx, syn);
                    edge_indices.push(edge);
                }
                net_ref.edges_indices.push(edge_indices);
            }
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

    pub fn update_output(&mut self, neuron_index: NodeIndex<u32>) -> f32 {
        let n = &mut self.neuron_graph[neuron_index];
        n.input_total = n.bias;
        for edge in self.neuron_graph.edges_directed(neuron_index, Direction::Incoming) {
            n.input_total += edge.weight().weight * (self.neuron_graph[edge.source()].output);
        }
        n.output = activation_from(n.activation)(n.input_total);
        n.output
    }

    pub fn forward_prop(&mut self, inputs: Vec<f32>) {
        // setup input layer
//        self.neuron_graph[0] = (0..inputs.len())
//            .map(|i| {
//                let mut n = Neuron::new(self.activation);
//                n.output = inputs[i];
//                n
//            }
//        ).collect::<Vec<Neuron>>();
//
//        for i in 1..self.neuron_graph.len() {
//            let layer = self.neuron_graph[i];
//            for mut node in layer {
//                node.update_output();
//            }
//        }
    }
}

