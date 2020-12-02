use derive_more::{From, Into};

#[derive(Debug, Copy, Clone, From, Into)]
pub struct Width(u32);

#[derive(Debug, Copy, Clone, From, Into)]
pub struct Height(u32);

#[derive(Debug, Copy, Clone, From, Into)]
pub struct Length(usize);

#[derive(Debug, Copy, Clone, From, Into)]
pub struct Pixel(u8);
impl Pixel {
    pub fn new(v: u8) -> Pixel {
        Pixel(v)
    }
}
impl PartialEq<Pixel> for u8 {
    fn eq(&self, other: &Pixel) -> bool {
        *self == other.0
    }
}

pub struct Image {
    pub width: Width,
    pub height: Height,
    pub length: Length,
    pub pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        let length = (width * height) as usize;
        let (ptr, len, cap) = pixels.into_raw_parts();
        let pixels = unsafe { Vec::from_raw_parts(ptr as *mut Pixel, len, cap) };
        Self {
            width: Width(width),
            height: Height(height),
            length: Length(length),
            pixels,
        }
    }

    #[inline]
    pub fn pixel(&mut self, pos: usize) -> u8 {
        self.pixels[pos * 4].into()
    }

    #[inline]
    pub fn paint(&mut self, target: usize, color: u8) {
        let p = Pixel::new(color);
        self.pixels[target] = p;
        self.pixels[target + 1] = p;
        self.pixels[target + 2] = p;
    }

    #[inline]
    pub fn height(&mut self) -> u32 {
        self.height.into()
    }

    #[inline]
    pub fn width(&mut self) -> u32 {
        self.width.into()
    }

    pub fn pixels(&self) -> Vec<u8> {
        let pixels = self.pixels.clone();
        let (ptr, len, cap) = pixels.into_raw_parts();
        unsafe { Vec::from_raw_parts(ptr as *mut u8, len, cap) }
    }
}
