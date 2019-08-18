pub enum Regularization {
    L1,
    L2
}

pub trait RegularizationFunction {
    fn output(weight: f32) -> f32;
    fn derivative(weight: f32) -> f32;
}

pub fn regularization_output(regularization: Regularization) -> impl Fn(f32) -> f32 {
    match regularization {
        Regularization::L1 => L1::output,
        Regularization::L2 => L2::output
    }
}

pub fn regularization_derivative(regularization: Regularization) -> impl Fn(f32) -> f32 {
    match regularization {
        Regularization::L1 => L1::derivative,
        Regularization::L2 => L2::derivative
    }
}

struct L1;
impl RegularizationFunction for L1 {
    fn output(weight: f32) -> f32 {
        weight.abs()
    }

    fn derivative(weight: f32) -> f32 {
        if weight < 0f32 {
            -1f32
        }
        else if weight > 0f32 {
            1f32
        }
        else {
            0f32
        }
    }
}

struct L2;
impl RegularizationFunction for L2 {
    fn output(weight: f32) -> f32 {
        0.5f32 * weight * weight
    }

    fn derivative(weight: f32) -> f32 {
        weight
    }
}
