extern crate wasm_bindgen;
extern crate rand;
extern crate nalgebra;
extern crate petgraph;

use wasm_bindgen::prelude::*;
mod activation;
mod neuron;
mod network;
mod synapse;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
