use rand::{ thread_rng, Rng };

pub struct Synapse {
    pub weight: f32,
    pub is_dead: bool
}

impl Synapse {
    pub fn new() -> Synapse {
        Synapse {
            weight: thread_rng().gen::<f32>(),
            is_dead: false
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