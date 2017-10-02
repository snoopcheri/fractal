use super::region::Region;
use super::mandelbrot_engine::MandelbrotEngine;


pub struct Mandelbrot {
    pub region: Region
}


impl Mandelbrot {
    pub fn new(region: Region) -> Mandelbrot {
        Mandelbrot { region }
    }

    pub fn calculate(&self, engine: &MandelbrotEngine) -> Vec<u8> {
        let mut pixels: Vec<u8> = vec![0 as u8; (self.region.width_in_pixels * self.region.height_in_pixels) as usize];

        engine.calculate(&self.region, &mut pixels);

        pixels
    }
}
