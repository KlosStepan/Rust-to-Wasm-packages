extern crate wasm_bindgen;
extern crate qrcode;
extern crate image;
extern crate base64;

use wasm_bindgen::prelude::*;
use qrcode::QrCode;
use std::io::Read;
use base64::{decode as b64decode, encode as b64encode};
use image::{load_from_memory, DynamicImage, ImageOutputFormat, Luma};
use rocket_contrib::json::Json;
use rqrr::PreparedImage;


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
    // Encode some data into bits.
    let code = QrCode::new(b"01234567").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    //image.save("/tmp/qrcode.png").unwrap();

    // You can also render it into a string.
    let string = code.render()
        .dark_color('#')
        .light_color(' ')
        .build();
    return format!("{}", string);

}
#[wasm_bindgen]
pub fn encode_qr_payment(
    text: String
    //width: Option<u32>,
    //height: Option<u32>,
//) -> Result<Json<String>, Json<String>> {
) -> String {
    //let width = width.unwrap_or(128);
    let width = 128;
    //let height = height.unwrap_or(128);
    let height = 128;

    if let Ok(qrcode) = QrCode::new(text.as_bytes()) {
        let qrcode_image_buffer = qrcode
            .render::<Luma<u8>>()
            .max_dimensions(width, height)
            .build();

        let qrcode_dynamic_image = DynamicImage::ImageLuma8(qrcode_image_buffer);

        let mut image_bytes: Vec<u8> = Vec::new();

        if let Ok(_v) = qrcode_dynamic_image.write_to(&mut image_bytes, ImageOutputFormat::Png) {
            //Ok(Json(b64encode(image_bytes)))
            return format!("{}", b64encode(image_bytes));
        } else {
            //Err(Json(String::from("Error: Cannot get image bytes")))
            return "error".to_string()
        }
    } else {
        //Err(Json(String::from("Error: Cannot encode this text")))
        return "error".to_string()
    }
}