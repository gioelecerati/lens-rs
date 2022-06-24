pub mod crypto;
pub mod graphql;
pub mod lens;
pub mod methods;
pub mod tests;
pub mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum Chain {
    Polygon,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum Net {
    Mumbai,
    Main,
}
