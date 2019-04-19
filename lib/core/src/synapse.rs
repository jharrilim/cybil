extern crate rand;
use rand::{ thread_rng, Rng };

use crate::neuron::Neuron;

pub struct Synapse<'n> {
    a: &'n Neuron,
    b: &'n Neuron,
    weight: f32
}

impl <'n> Synapse<'n> {
    pub fn new(a: &'n Neuron, b: &'n Neuron) -> Synapse<'n> {
        Synapse { a, b, weight: thread_rng().gen() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;

    pub fn create_synapse() {
        let n1 = Neuron { location: Point::new(3f32,2f32) };
        let n2 = Neuron { location: Point::new(40f32, 70f32) };

        let syn = Synapse::new(&n1, &n2);
    }
}