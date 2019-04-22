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

    #[test]
    pub fn create_synapse() {
        Synapse::new();
    }
}