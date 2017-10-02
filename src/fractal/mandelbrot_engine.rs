use super::region::Region;
use super::pixel::Pixel;

pub trait MandelbrotEngine {
    fn calculate(&self, region: &Region, pixels: &mut Vec<u8>) {
        if self.should_calculate_in_parallel() {
            self.calculate_in_parallel(region, pixels);
        } else {
            self.calculate_serially(region, pixels);
        }
    }


    fn set_pixel(&self, pixel: &Pixel, color: u8, width_in_pixels: u32, pixel_offset: u32, pixels: &mut [u8]) {
        pixels[(pixel.y * width_in_pixels + pixel.x - pixel_offset) as usize] = color;
    }


    fn should_calculate_in_parallel(&self) -> bool;
    fn calculate_serially(&self, region: &Region, pixels: &mut Vec<u8>);
    fn calculate_in_parallel(&self, region: &Region, pixels: &mut Vec<u8>);
}
