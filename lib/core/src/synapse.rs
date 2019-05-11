use rand::{ thread_rng, Rng };

pub struct Synapse {
    pub weight: f32,
    pub is_dead: bool,
    pub error_derivative: f32,
    pub accumulated_error_derivative: f32,
    pub num_accumulated_error_derivative: f32
}

impl Synapse {
    pub fn new() -> Synapse {
        Synapse {
            weight: thread_rng().gen::<f32>(),
            is_dead: false,
            error_derivative: 0f32,
            accumulated_error_derivative: 0f32,
            num_accumulated_error_derivative: 0f32
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn create_synapse() {
        Synapse::new();
    }
}