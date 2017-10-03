use num::complex::Complex64;

use super::region::Region;
use super::pixel::Pixel;
use super::mandelbrot_engine::MandelbrotEngine;


pub struct Mandelbrot {
    pub region: Region,
    pub width: u32,
    pub height: u32,
    pub max_iterations: u8,
}


impl Mandelbrot {
    pub fn new(region: Region, width: u32, height: u32, max_iterations: u8) -> Mandelbrot {
        Mandelbrot { region, width, height, max_iterations }
    }


    pub fn calculate(&self, engine: &MandelbrotEngine) -> Vec<u8> {
        let mut pixels: Vec<u8> = vec![0 as u8; (self.width * self.height) as usize];

        engine.calculate(&self, &mut pixels);

        pixels
    }


    pub fn point_for_pixel(&self, pixel: &Pixel) -> Complex64 {
        Complex64 {
            re: self.region.min_re() + (pixel.x as f64) * self.region.width() / (self.width as f64),
            im: self.region.min_im() + (pixel.y as f64) * self.region.height() / (self.height as f64),
        }
    }
}
