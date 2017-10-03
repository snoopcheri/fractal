use rayon::prelude::*;

use super::mandelbrot_engine::MandelbrotEngine;
use super::region::Region;
use super::window::Window;
use super::window_iterator::{WindowLineIterator, WindowAreaIterator};
use super::escape_time::EscapeTime;
use super::pixel::Pixel;
use super::pixel_band::PixelBand;


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

        let mut pixel_band = PixelBand::new(pixels, 0);
        calculate_for_pixel_iterator(region, pixel_iterator, &mut pixel_band);
    }


    fn calculate_in_parallel(&self, region: &Region, pixels: &mut Vec<u8>) {
        let window = Window::new(0, 0, region.width_in_pixels, region.height_in_pixels);

        let chunk_size = region.width_in_pixels as usize;
        let workload: Vec<(PixelBand, WindowLineIterator)> = pixels
            .chunks_mut(chunk_size)
            .enumerate()
            .map(|(i, pixel_chunk)| {
                (
                    ith_pixel_band(i, pixel_chunk, chunk_size),
                    ith_pixel_iterator(i, &window),
                )
            })
            .collect();

        workload.into_par_iter()
            .for_each(|(mut pixel_band, pixel_iterator)| {
                calculate_for_pixel_iterator(region, pixel_iterator, &mut pixel_band);
            });
    }
}


fn ith_pixel_band(i: usize, pixel_chunk: &mut [u8], chunk_size: usize) -> PixelBand {
    PixelBand::new(pixel_chunk, i * chunk_size)
}


fn ith_pixel_iterator(i: usize, window: &Window) -> WindowLineIterator {
    WindowLineIterator::new(window, i as u32)
}


fn calculate_for_pixel_iterator<I>(region: &Region, pixel_iterator: I, pixel_band: &mut PixelBand)
    where I: Iterator<Item=Pixel>
{
    for pixel in pixel_iterator {
        let point = region.point_for_pixel(&pixel);
        let color = point.escape_time(region.max_iterations);

        pixel_band.set_color_of_pixel(color, &pixel, region.width_in_pixels);
    }
}
