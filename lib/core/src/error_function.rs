pub enum ErrorFunc {
    Square
}

pub fn error_function_from(error_func: &ErrorFunc) -> impl Fn(f32, f32) -> f32 {
    match *error_func {
        ErrorFunc::Square => Square::call,
    }
}

pub fn error_derivative(error_func: &ErrorFunc) -> impl Fn(f32, f32) -> f32 {
    match *error_func {
        ErrorFunc::Square => Square::first_prime
    }
}

trait ErrorFunction {
    fn call(output: f32, target: f32) -> f32;
    fn first_prime(output: f32, target: f32) -> f32;
}


pub struct Square;
impl ErrorFunction for Square {
    fn call(output: f32, target: f32) -> f32 {
        0.5f32 * (output - target).powi(2)
    }

    fn first_prime(output: f32, target: f32) -> f32 {
        output - target
    }
}
