use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SecureOrder {
    subtotal: f64,
    tax_rate: f64,
}

#[wasm_bindgen]
impl SecureOrder {
    #[wasm_bindgen(constructor)]
    pub fn new(tax_rate: f64) -> Result<SecureOrder, JsValue> {
        Ok(SecureOrder {
            subtotal: 0.0,
            tax_rate,
        })
    }

    // Simulate secure price calculation in TEE
    #[wasm_bindgen]
    pub fn calculate_total(&self) -> f64 {
        let tax = self.subtotal * self.tax_rate;
        (self.subtotal + tax).round() * 100.0 / 100.0 // Round to 2 decimal places
    }

    // Simulate secure item addition in TEE
    #[wasm_bindgen]
    pub fn add_item(&mut self, price: f64, quantity: u32) {
        self.subtotal += price * quantity as f64;
    }

    // Get subtotal (for display purposes)
    #[wasm_bindgen]
    pub fn get_subtotal(&self) -> f64 {
        self.subtotal
    }
}

// Simulate secure hash generation for order ID
#[wasm_bindgen]
pub fn generate_order_id(timestamp: f64) -> String {
    format!("ORDER-{}-TEE", (timestamp as u64 % 10000))
}

// Ensure proper initialization
#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
} 