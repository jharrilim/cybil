use crate::neuron::Neuron;

pub struct Synapse<'neuron> {
    a: &'neuron Neuron,
    b: &'neuron Neuron
}

impl Synapse {
    pub fn new(&a: Neuron, &b: Neuron) -> Synapse {
        Synapse { a, b }
    }
    
}