use crate::image::Image;
use crate::processor::Processor;

impl Processor {
    pub fn draw_frame(image: &mut Image) {
        let width: u32 = image.width.into();
        let height: u32 = image.height.into();

        // paint to top and bottom
        for y in [0, height - 1].iter() {
            for x in 0..width {
                let p = ((y * width + x) * 4) as usize;
                image.paint(p, 255);
            }
        }

        // paint to left and right
        for x in [0, width - 1].iter() {
            for y in 0..height {
                let p = ((y * width + x) * 4) as usize;
                image.paint(p, 255);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_frame() {
        let pixels = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];
        let expect = vec![
            255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 0, 0, 0, 0,
            255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0,
        ];
        let mut image = Image::new(3, 3, pixels);

        Processor::draw_frame(&mut image);

        assert_eq!(expect, image.pixels());
    }
}
