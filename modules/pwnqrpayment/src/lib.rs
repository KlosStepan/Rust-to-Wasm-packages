use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greetStatic() -> String {
    "Hello from Rust!".to_string()
}

#[wasm_bindgen]
pub fn generateQRPayment(payment: &str) -> String {
    format!("generateQRPayment({})", payment)
}