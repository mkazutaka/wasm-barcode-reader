use crate::image::{Image, Pixel};
use crate::processor::Processor;
use line_drawing::Bresenham;

pub type Position = (i64, i64);

impl Processor {
    pub fn draw_line(image: &mut Image, start: Position, end: Position) {
        for (x, y) in Bresenham::new(start, end) {
            let position = ((y * image.width() as i64 + x) * 4) as usize;

            image.pixels[position] = Pixel::new(255);
            image.pixels[position + 1] = Pixel::new(0);
            image.pixels[position + 2] = Pixel::new(255);
        }
    }

    pub fn extract_line(image: &mut Image, start: Position, end: Position) -> Vec<u8> {
        let mut line = vec![];
        for (x, y) in Bresenham::new(start, end) {
            let position = ((y * image.width() as i64 + x) * 4) as usize;
            let v: u8 = image.pixels[position].into();
            line.push(v);
            line.push(v);
            line.push(v);
            line.push(v);
        }

        line
    }
}
