use rayon::prelude::*;

use super::mandelbrot_engine::MandelbrotEngine;
use super::region::Region;
use super::window::Window;
use super::window_iterator::{WindowLineIterator, WindowAreaIterator};
use super::escape_time::EscapeTime;
use super::pixel::Pixel;


pub struct SimpleMandelbrotEngine {
    in_parallel: bool,
}

impl SimpleMandelbrotEngine {
    pub fn new(in_parallel: bool) -> SimpleMandelbrotEngine {
        SimpleMandelbrotEngine { in_parallel }
    }
}


impl MandelbrotEngine for SimpleMandelbrotEngine {
    fn should_calculate_in_parallel(&self) -> bool {
        self.in_parallel
    }


    fn calculate_serially(&self, region: &Region, pixels: &mut Vec<u8>) {
        let window = Window::new(0, 0, region.width_in_pixels, region.height_in_pixels);
        let pixel_iterator = WindowAreaIterator::new(&window);

        calculate_for_pixel_iterator(region, pixel_iterator, 0, pixels);
    }


    fn calculate_in_parallel(&self, region: &Region, pixels: &mut Vec<u8>) {
        let window = Window::new(0, 0, region.width_in_pixels, region.height_in_pixels);

        let pixel_bands: Vec<(usize, &mut [u8])> = pixels
            .chunks_mut(region.width_in_pixels as usize)
            .enumerate()
            .collect();

        pixel_bands.into_par_iter()
            .for_each(|(i, pixel_band)| {
                let pixel_iterator = WindowLineIterator::new(&window, i as u32);
                let pixel_offset = (i as u32) * region.width_in_pixels;

                calculate_for_pixel_iterator(region, pixel_iterator, pixel_offset, pixel_band);
            });
    }
}


fn calculate_for_pixel_iterator<I>(region: &Region, pixel_iterator: I, pixel_offset: u32, pixels: &mut [u8])
    where I: Iterator<Item=Pixel>
{
    for pixel in pixel_iterator {
        let point = region.point_for_pixel(&pixel);
        let escape = point.escape_time(region.max_iterations);

        pixel.draw(escape, region.width_in_pixels, pixel_offset, pixels);
    }
}
