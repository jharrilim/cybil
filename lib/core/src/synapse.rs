
use rand::{ thread_rng, Rng };

use crate::neuron::Neuron;

pub struct Synapse {
    a: Neuron,
    b: Neuron,
    weight: f32
}

impl Synapse {
    pub fn new(a: Neuron, b: Neuron) -> Synapse {
        Synapse { a, b, weight: thread_rng().gen::<f32>() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use nalgebra::geometry::Vector2;
    use crate::activation::Activation;

    pub fn create_synapse() {
        let n1 = Neuron::new(Activation::ReLU);
        let n2 = Neuron::new(Activation::ReLU);

        let syn = Synapse::new(&n1, &n2);
    }
}