extern crate wasm_bindgen;
extern crate rand;
extern crate petgraph;

use wasm_bindgen::prelude::*;
use crate::activation::Activation;
use crate::error_function::ErrorFunc;
use crate::regularization::Regularization;

mod activation;
mod neuron;
mod network;
mod synapse;
mod error_function;
mod regularization;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn network(shape: Vec<i32>) -> network::Network {
    network::Network::create(
        shape,
        Option::Some(Activation::ReLU),
        Option::Some(Activation::ReLU),
        Option::Some(ErrorFunc::Square),
        Option::Some(Regularization::L1)
    )
}