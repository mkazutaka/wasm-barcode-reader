#![feature(vec_into_raw_parts)]

pub mod decoder;
pub mod detector;
pub mod image;
pub mod processor;
pub mod reader;
pub mod utils;
use crate::image::Image;
use crate::processor::Processor;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn detect(width: u32, height: u32, pixels: Vec<u8>) -> String {
    let mut image = Image::new(width, height, pixels);
    // let _reader = reader::BarcodeReader::new(image);
    let cluster = detector::detect(&mut image);

    let (start, end) = cluster.center();
    let line = Processor::extract_line(&mut image, start, end);

    let result = decoder::ean_13::decode(&mut line.iter().step_by(4));
    match result {
        Some(result) => {
            format!(
                "{} {} {} {} {} {} {} {} {} {} {} {} {}",
                result[0],
                result[1],
                result[2],
                result[3],
                result[4],
                result[5],
                result[6],
                result[7],
                result[8],
                result[9],
                result[10],
                result[11],
                result[12]
            )
        }
        _ => {
            format!("")
        }
    }
}
