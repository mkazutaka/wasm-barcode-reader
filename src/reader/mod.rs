use crate::image::Image;
use crate::processor::Processor;
// mod contour;
// mod tracer;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Label {
    Unmarked,
    Marked(u32),
    InnerMarked(u32),
    OutsideEdge,
    InsideEdge,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn new(b: u8) -> Color {
        match b {
            255 => Color::White,
            _ => Color::Black,
        }
    }
}

pub struct BarcodeReader {
    image: Image,
}

impl BarcodeReader {
    pub fn new(image: Image) -> BarcodeReader {
        BarcodeReader { image }
    }

    pub fn decode(&mut self) {
        Processor::threshold(&mut self.image);
        Processor::draw_frame(&mut self.image);
    }
}
