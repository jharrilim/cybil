use crate::neuron::Neuron;

pub struct Synapse<'n> {
    a: &'n Neuron,
    b: &'n Neuron,
    weight: f32
}

impl <'n> Synapse<'n> {
    pub fn new(a: &'n Neuron, b: &'n Neuron) -> Synapse<'n> {
        Synapse { a, b, weight: 1f32 }
    }

}