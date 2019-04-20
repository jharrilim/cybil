use nalgebra::base::Vector2;

pub struct Neuron {
    pub location: Vector2<f32>
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron {
            location: Vector2::new(0f32, 0f32)
        }
    }
}