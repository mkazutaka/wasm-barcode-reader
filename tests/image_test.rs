use image::GenericImageView;
use wasm_barcode_reader::detector::detect;
use wasm_barcode_reader::image::{Image, Pixel};

use wasm_barcode_reader::decoder::ean_13::decode;
use wasm_barcode_reader::processor::Processor;
use wasm_barcode_reader::reader::Label;

#[test]
fn test_threshold() {
    let input = "./tests/fixtures/image-001.png";
    let output = "./tests/fixtures/threshold-001.png";
    let opened = image::open(input).expect("Faild to open Image");
    let mut target = Image::new(opened.width(), opened.height(), opened.to_bytes());

    Processor::threshold(&mut target);

    image::save_buffer(
        output,
        &target.pixels(),
        opened.width(),
        opened.height(),
        image::ColorType::Rgba8,
    )
    .expect("Failed to save Image");
}

#[test]
fn test_draw_frame() {
    let input = "./tests/fixtures/image-001.png";
    let output = "./tests/fixtures/surround-001.png";
    let opened = image::open(input).expect("Faild to open Image");
    let mut target = Image::new(opened.width(), opened.height(), opened.to_bytes());

    Processor::draw_frame(&mut target);

    image::save_buffer(
        output,
        &target.pixels(),
        opened.width(),
        opened.height(),
        image::ColorType::Rgba8,
    )
    .expect("Failed to save Image");
}

#[test]
fn test_countour_for_image() {
    let input = "./tests/fixtures/image-001.png";
    let output = "./tests/fixtures/countour-001.png";
    let opened = image::open(input).expect("Faild to open Image");
    let mut target = Image::new(opened.width(), opened.height(), opened.to_bytes());
    let length: usize = target.length.into();
    let _label_image = vec![Label::Unmarked; length];

    let labels = Processor::draw_contour(&mut target);
    for (index, label) in labels.iter().enumerate() {
        let index = index * 4;
        match label {
            Label::Marked(v) => match v % 9 {
                0 => {
                    target.pixels[index] = Pixel::new(0);
                    target.pixels[index + 1] = Pixel::new(200);
                    target.pixels[index + 2] = Pixel::new(200);
                }
                1 => {
                    target.pixels[index] = Pixel::new(200);
                    target.pixels[index + 1] = Pixel::new(0);
                    target.pixels[index + 2] = Pixel::new(200);
                }
                2 => {
                    target.pixels[index] = Pixel::new(200);
                    target.pixels[index + 1] = Pixel::new(200);
                    target.pixels[index + 2] = Pixel::new(0);
                }
                3 => {
                    target.pixels[index] = Pixel::new(200);
                    target.pixels[index + 1] = Pixel::new(0);
                    target.pixels[index + 2] = Pixel::new(0);
                }
                4 => {
                    target.pixels[index] = Pixel::new(0);
                    target.pixels[index + 1] = Pixel::new(200);
                    target.pixels[index + 2] = Pixel::new(0);
                }
                5 => {
                    target.pixels[index] = Pixel::new(0);
                    target.pixels[index + 1] = Pixel::new(0);
                    target.pixels[index + 2] = Pixel::new(200);
                }
                6 => {
                    target.pixels[index] = Pixel::new(100);
                    target.pixels[index + 1] = Pixel::new(100);
                    target.pixels[index + 2] = Pixel::new(0);
                }
                7 => {
                    target.pixels[index] = Pixel::new(0);
                    target.pixels[index + 1] = Pixel::new(100);
                    target.pixels[index + 2] = Pixel::new(100);
                }
                8 => {
                    target.pixels[index] = Pixel::new(100);
                    target.pixels[index + 1] = Pixel::new(0);
                    target.pixels[index + 2] = Pixel::new(100);
                }
                _ => {
                    target.pixels[index] = Pixel::new(200);
                    target.pixels[index + 1] = Pixel::new(200);
                    target.pixels[index + 2] = Pixel::new(200);
                }
            },
            Label::InnerMarked(_v) => {}
            _ => {}
        }
    }

    image::save_buffer(
        output,
        &target.pixels(),
        opened.width(),
        opened.height(),
        image::ColorType::Rgba8,
    )
    .expect("Failed to save Image");
}

#[test]
fn test_detection_for_image() {
    let input = "./tests/fixtures/image-001.png";
    let output = "./tests/fixtures/detect-001.png";
    let opened = image::open(input).expect("Faild to open Image");
    let mut target = Image::new(opened.width(), opened.height(), opened.to_bytes());

    let cluster = detect(&mut target);

    for component in cluster.components.iter() {
        for (x, y) in component.contour.iter() {
            let pos = (((y * opened.width() as i64) + x) * 4) as usize;
            target.pixels[pos] = Pixel::new(0);
            target.pixels[pos] = Pixel::new(255);
            target.pixels[pos] = Pixel::new(255);
        }
    }

    let (s, e) = cluster.center();
    Processor::draw_line(&mut target, s, e);

    image::save_buffer(
        output,
        &target.pixels(),
        opened.width(),
        opened.height(),
        image::ColorType::Rgba8,
    )
    .expect("Failed to save Image");
}

#[test]
fn test_decode() {
    let input = "./tests/fixtures/image-001.png";
    let opened = image::open(input).expect("Faild to open Image");
    let mut target = Image::new(opened.width(), opened.height(), opened.to_bytes());

    let cluster = detect(&mut target);
    let (s, e) = cluster.center();
    let line = Processor::extract_line(&mut target, s, e);

    let result = decode(&mut line.iter().step_by(4));

    assert_eq!([9, 7, 8, 4, 9, 0, 8, 6, 8, 6, 0, 0, 9], result.unwrap());
}
