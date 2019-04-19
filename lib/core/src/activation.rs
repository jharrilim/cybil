use std::intrinsics::powf32;

pub fn sigmoid(a: f32) -> f32 {
    1f32 / (1f32 + a.powf(-1f32))
}


#[cfg!(test)]
mod test {
    use super::activation::sigmoid;

    pub fn test() {
        sigmoid(5f32)
    }
}