use num::complex::Complex64;

use super::pixel::Pixel;


#[derive(Debug)]
pub enum RegionType {
    Default,
    SeaHorseValley,
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Region {
    min_re: f64,
    min_im: f64,
    region_width: f64,
    region_height: f64,
    pub width_in_pixels: u32,
    pub height_in_pixels: u32,
    pub max_iterations: u8,
}

impl Region {
    pub fn new(min_re: f64, max_re: f64, min_im: f64, max_im: f64, width_in_pixels: u32, height_in_pixels: u32, max_iterations: u8) -> Region {
        let region_width = max_re - min_re;
        let region_height = max_im - min_im;

        Region { min_re, min_im, region_width, region_height, width_in_pixels, height_in_pixels, max_iterations }
    }

    pub fn new_for_center(center: Complex64, radius: f64, width_in_pixels: u32, height_in_pixels: u32, max_iterations: u8) -> Region {
        Region::new(center.re - radius, center.re + radius, center.im - radius, center.im + radius, width_in_pixels, height_in_pixels, max_iterations)
    }

    pub fn new_for_type(region_type: RegionType, width_in_pixels: u32, height_in_pixels: u32, max_iterations: u8) -> Region {
        match region_type {
            RegionType::Default => Region::new(-2.0, 1.0, -1.0, 1.0, width_in_pixels, height_in_pixels, max_iterations),
            RegionType::SeaHorseValley => Region::new_for_center(Complex64::new(-0.74548, 0.11669), 0.01276, width_in_pixels, height_in_pixels, max_iterations)
        }
    }

    pub fn point_for_pixel(&self, pixel: &Pixel) -> Complex64 {
        Complex64 {
            re: self.min_re + (pixel.x as f64) * self.region_width / (self.width_in_pixels as f64),
            im: self.min_im + (pixel.y as f64) * self.region_height / (self.height_in_pixels as f64),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_region() {
        // act
        let region = Region::new(-2.0, 2.0, -1.0, 1.0, 400, 200, 50);

        // assert
        assert_eq!(region.min_re, -2.0);
        assert_eq!(region.min_im, -1.0);
        assert_eq!(region.region_width, 4.0);
        assert_eq!(region.region_height, 2.0);
        assert_eq!(region.width_in_pixels, 400);
        assert_eq!(region.height_in_pixels, 200);
        assert_eq!(region.max_iterations, 50);
    }

    #[test]
    fn point_for_pixel() {
        // arrange
        let region = Region::new(-2.0, 2.0, -1.0, 1.0, 400, 200, 50);

        // act + assert
        assert_eq!(region.point_for_pixel(&Pixel { x: 0, y: 0 } ), Complex64::new(-2.0, -1.0));
        assert_eq!(region.point_for_pixel(&Pixel { x: 200, y: 100 } ), Complex64::new(0.0, 0.0));
    }
}