use crate::image::Image;
use crate::processor::Processor;

impl Processor {
    pub fn threshold(image: &mut Image) {
        let mut histgram: [f64; 256] = [0.0; 256];
        let end = image.length.into();

        for i in 0..end {
            let pixel: u8 = image.pixels[i * 4].into();
            histgram[pixel as usize] += 1.0;
        }

        let mut s_max = (0, 0.0);
        for th in 0..255 {
            // Calculate M1: 0 > Threshold
            let mut w1 = 0.0;
            let mut sum1 = 0.0;
            for i in 0..th {
                w1 += histgram[i];
                sum1 += histgram[i] * i as f64;
            }
            let m1 = if w1 == 0.0 { 0.0 } else { sum1 / w1 };

            // Calculate M2: Threshold < 0
            let mut w2 = 0.0;
            let mut sum2 = 0.0;
            for i in th..256 {
                w2 += histgram[i];
                sum2 += histgram[i] * i as f64;
            }
            let m2 = if w2 == 0.0 { 0.0 } else { sum2 / w2 };

            // Result
            let s = w1 * w2 * (m1 - m2).powf(2.0);
            if s > s_max.1 {
                s_max = (th, s);
            }
        }

        let threshold = s_max.0 as u8;
        for i in 0..end {
            let p = i * 4;
            let pixel: u8 = image.pixels[p].into();
            let color = if pixel < threshold { 0 } else { 255 };
            image.paint(p, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threshold() {
        let pixels = vec![0, 0, 0, 0, 1, 0, 0, 0, 254, 0, 0, 0, 255, 0, 0, 0];
        let expect = vec![0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 0, 255, 255, 255, 0];
        let mut image = Image::new(2, 2, pixels);

        Processor::threshold(&mut image);

        assert_eq!(expect, image.pixels());
    }
}
