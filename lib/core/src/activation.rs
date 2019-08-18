
#[derive(Copy, Clone)]
pub enum Activation {
    Sigmoid,
    Tanh,
    ReLU,
    Linear
}


pub fn activation_from(activation: Activation) -> impl Fn(f32) -> f32 {
    match activation {
        Activation::Sigmoid => Sigmoid::call,
        Activation::Tanh => Tanh::call,
        Activation::ReLU => Relu::call,
        Activation::Linear => Linear::call
    }
}


pub fn activation_derivative(activation: Activation) -> impl Fn(f32) -> f32 {
    match activation {
        Activation::Sigmoid => Sigmoid::first_prime,
        Activation::Tanh => Tanh::first_prime,
        Activation::ReLU => Relu::first_prime,
        Activation::Linear => Linear::first_prime
    }
}


pub trait ActivationFunction {
    fn call(a: f32) -> f32;
    fn first_prime(a: f32) -> f32;
}


pub struct Sigmoid;
impl ActivationFunction for Sigmoid {
    fn call(a: f32) -> f32 {
        1f32 / (1f32 + a.powf(-1f32))
    }
    fn first_prime(a: f32) -> f32 {
        Sigmoid::call(a) * (1f32 - Sigmoid::call(a))
    }
}


pub struct Tanh;
impl ActivationFunction for Tanh {
    fn call(a: f32) -> f32 {
        a.tanh()
    }

    fn first_prime(a: f32) -> f32 {
        let tanh_a = a.tanh();
        1f32 - tanh_a * tanh_a
    }
}


pub struct Relu;
impl ActivationFunction for Relu {
    fn call(a: f32) -> f32 {
        if a > 0f32 { a } else { 0f32 }
    }

    fn first_prime(a: f32) -> f32 {
        if a <= 0f32 { 0f32 } else { 1f32 }
    }
}


pub struct Linear;
impl ActivationFunction for Linear {
    fn call(a: f32) -> f32 {
        a
    }

    fn first_prime(_a: f32) -> f32 {
        1f32
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn activation_from_returns_correct_fn() {
        assert_eq!(activation_from(Activation::Sigmoid)(5f32), Sigmoid::call(5f32));
        assert_eq!(activation_derivative(Activation::Sigmoid)(5f32), Sigmoid::first_prime(5f32));
        
        assert_eq!(activation_from(Activation::Linear)(100f32), Linear::call(100f32));
        assert_eq!(activation_derivative(Activation::Linear)(100f32), Linear::first_prime(100f32));

        assert_eq!(activation_from(Activation::ReLU)(42f32), Relu::call(42f32));
        assert_eq!(activation_derivative(Activation::ReLU)(9090f32), Relu::first_prime(9090f32));

        assert_eq!(activation_from(Activation::Tanh)(0.101238f32), Tanh::call(0.101238f32));
        assert_eq!(activation_derivative(Activation::Tanh)(0.101238f32), Tanh::first_prime(0.101238f32));
    }
}