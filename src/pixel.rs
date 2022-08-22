#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub fn pixels_eq(pixel1: Pixel, pixel2: Pixel) -> bool {
    if pixel1.r == pixel2.r && pixel1.g == pixel2.g && pixel1.b == pixel2.b && pixel1.a == pixel2.a {
        return true;
    }
    return false;
}