#[derive(Debug)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
}


impl Pixel {
    pub fn draw(&self, color: u8, region_width: u32, offset_in_pixels_slice: u32, pixels: &mut [u8]) {
        let idx = (self.y * region_width) + self.x - offset_in_pixels_slice;

        pixels[idx as usize] = color;
    }
}
