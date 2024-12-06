use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_hash(input: &str) -> String {
    format!("Hashed: {}", input)
}

#[wasm_bindgen]
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
} 