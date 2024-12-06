use wasm_bindgen::prelude::*;

// Simulate TEE memory regions
#[wasm_bindgen]
pub struct SecureMemory {
    encrypted_data: Vec<u8>,
}

#[wasm_bindgen]
impl SecureMemory {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SecureMemory, JsValue> {
        Ok(SecureMemory {
            encrypted_data: Vec::new(),
        })
    }

    // Simulate secure storage
    pub fn store_data(&mut self, data: &[u8]) {
        self.encrypted_data = data.to_vec();
    }
}

#[wasm_bindgen]
pub struct SecureOrder {
    subtotal: f64,
    tax_rate: f64,
    secure_memory: SecureMemory,
}

#[wasm_bindgen]
impl SecureOrder {
    #[wasm_bindgen(constructor)]
    pub fn new(tax_rate: f64) -> Result<SecureOrder, JsValue> {
        Ok(SecureOrder {
            subtotal: 0.0,
            tax_rate,
            secure_memory: SecureMemory::new()?,
        })
    }

    // Simulate secure calculation in TEE
    #[wasm_bindgen]
    pub fn calculate_total(&self) -> f64 {
        // Simulate entering TEE environment
        let tax = self.subtotal * self.tax_rate;
        let total = (self.subtotal + tax).round() * 100.0 / 100.0;
        
        // Store sensitive data in secure memory
        self.secure_memory.store_data(&total.to_be_bytes());
        
        total
    }

    #[wasm_bindgen]
    pub fn add_item(&mut self, price: f64, quantity: u32) {
        // Simulate secure operation in TEE
        self.subtotal += price * quantity as f64;
    }

    #[wasm_bindgen]
    pub fn get_subtotal(&self) -> f64 {
        self.subtotal
    }
}

// Simulate secure hash generation in TEE
#[wasm_bindgen]
pub fn generate_order_id(timestamp: f64) -> String {
    // Simulate TEE-protected operation
    format!("ORDER-{}-TEE", (timestamp as u64 % 10000))
}

#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
} 