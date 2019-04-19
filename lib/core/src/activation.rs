use std::intrinsics::powf32;

pub fn sigmoid(a: f32) -> f32 {
    1f32 / (1f32 + a.powf(-1f32))
}

pub fn sigmoid_prime(a: f32) -> f32 {
    sigmoid(a) * (1f32 - sigmoid(a))
}


#[cfg!(test)]
mod test {
    use super::activation::sigmoid;

    pub fn test() {
        sigmoid(5f32)
    }
}