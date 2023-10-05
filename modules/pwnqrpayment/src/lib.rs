extern crate qrcode;
extern crate image;
extern crate base64;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use qrcode::QrCode;
use qrcode::render::svg::Color; // Explicitly import Color
use image::{DynamicImage, ColorType};
use base64::write::EncoderWriter;
use base64::STANDARD;

//TODO React stuff


//#[wasm_bindgen]
//pub fn greetStatic() -> String {
//    "Hello from Rust!".to_string()
//}

#[wasm_bindgen]
pub fn generateQRPayment(payment: &str) -> String {
    format!("generateQRPayment({})", payment)
}
#[wasm_bindgen]
pub fn generate_dummy_qr() -> String {
    let text = "Hello, QR code!";

    // Create a QR code
    let code = QrCode::new(text).unwrap();

    // Specify width and height of the image
    let width = 250u32;
    let height = 250u32;

    // Get the SVG string from the QR code
    let svg = code.render::<Color>().finish();

    // Create an image from the SVG string
    let image: DynamicImage = image::load_from_memory(svg.as_bytes()).unwrap();

    // Resize the image to the specified width and height
    let resized_image = image.resize(width, height, image::imageops::FilterType::Nearest);

    // Encode the resized image as PNG and convert it to a base64 string
    let mut base64_encoder = EncoderWriter::new(Vec::new(), STANDARD);
    resized_image.write_to(&mut base64_encoder, image::ImageOutputFormat::Png).unwrap();
    let base64_string = base64_encoder.finish().unwrap();

    // Return the base64-encoded image as a string
    format!("Base64-encoded image:\n{}", String::from_utf8(base64_string).unwrap())
}