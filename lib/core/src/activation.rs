use nalgebra::max;

pub fn sigmoid(a: f32) -> f32 {
    1f32 / (1f32 + a.powf(-1f32))
}

pub fn sigmoid_prime(a: f32) -> f32 {
    sigmoid(a) * (1f32 - sigmoid(a))
}

pub fn tanh(a: f32) -> f32 {
    a.tanh()
}

pub fn tanh_prime(a: f32) -> f32 {
    let tanh_a = a.tanh();

    1 - tanh_a * tanh_a
}

pub fn relu(a: f32) -> f32 {
    max(0f32, a)
}

pub fn relu_prime (a: f32) -> f32 {
    if a <= 0f32 { 0f32 } else { 1f32 }
}

pub fn linear(a: f32) -> f32 {
    a
}

pub fn linear_prime(a: f32) -> f32 {
    1f32
}


#[cfg(test)]
mod test {
    use super::*;

    pub fn test() {
        sigmoid(5f32);
    }
}