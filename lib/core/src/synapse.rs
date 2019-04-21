use rand::{ thread_rng, Rng };

pub struct Synapse {
    pub weight: f32
}

impl Synapse {
    pub fn new() -> Synapse {
        Synapse { weight: thread_rng().gen::<f32>() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use nalgebra::geometry::Vector2;
    use crate::activation::Activation;

    pub fn create_synapse() {
        let syn = Synapse::new();
    }
}