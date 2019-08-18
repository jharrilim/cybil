use rand::{ thread_rng, Rng };
use crate::regularization::Regularization;

pub struct Synapse {
    pub weight: f32,
    pub is_dead: bool,
    pub error_der: f32,
    pub accumulated_error_der: f32,
    pub total_accumulated_error_der: f32,
    pub regularization: Regularization
}

impl Synapse {
    pub fn new(regularization: Regularization) -> Synapse {
        Synapse {
            weight: thread_rng().gen::<f32>(),
            is_dead: false,
            error_der: 0f32,
            accumulated_error_der: 0f32,
            total_accumulated_error_der: 0f32,
            regularization
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn create_synapse() {
        Synapse::new(Regularization::L1);
    }
}