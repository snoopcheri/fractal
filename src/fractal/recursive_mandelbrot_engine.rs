use rayon::prelude::*;
use num_cpus;

use super::mandelbrot_engine::MandelbrotEngine;
use super::region::Region;
use super::window::Window;
use super::pixel::Pixel;
use super::pixel_band::PixelBand;
use super::window_iterator::WindowAreaIterator;
use super::window_iterator::WindowBorderIterator;
use super::escape_time::EscapeTime;


pub struct RecursiveMandelbrotEngine {
    in_parallel: bool,
    band_height: u32,
}

impl RecursiveMandelbrotEngine {
    pub fn new(in_parallel: bool, band_height: u32) -> RecursiveMandelbrotEngine {
        RecursiveMandelbrotEngine { in_parallel, band_height }
    }
}


impl MandelbrotEngine for RecursiveMandelbrotEngine {
    fn should_calculate_in_parallel(&self) -> bool {
        self.in_parallel
    }


    fn calculate_serially(&self, region: &Region, pixels: &mut Vec<u8>) {
        let window = Window::new(0, 0, region.width_in_pixels, region.height_in_pixels);
        let mut pixel_band = PixelBand::new(pixels, 0);

        calculate_recursive(region, &window, &mut pixel_band);
    }


    fn calculate_in_parallel(&self, region: &Region, pixels: &mut Vec<u8>) {
        verify_band_height_for_region(self.band_height, region);

        let band_width = (region.width_in_pixels * self.band_height) as usize;
        let workload: Vec<(PixelBand, Window)> = pixels
            .chunks_mut(band_width)
            .enumerate()
            .map(|(i, pixel_chunk)| {
                let current_band_height = pixel_chunk.len() as u32 / region.width_in_pixels;

                (
                    ith_pixel_band(i, pixel_chunk, band_width),
                    ith_window(i, region.width_in_pixels, self.band_height, current_band_height),
                )
            })
            .collect();

        workload.into_par_iter()
            .for_each(|(mut pixel_band, window)| {
                calculate_recursive(region, &window, &mut pixel_band);
            });
    }
}


fn verify_band_height_for_region(band_height: u32, region: &Region) {
    let number_of_bands = ((region.height_in_pixels as f64) / (band_height as f64)).ceil() as u32;
    assert!(number_of_bands > 0);

    let last_band_height = region.height_in_pixels - (number_of_bands - 1) * band_height;
    assert!(last_band_height >= 8);

    let number_of_cpus = num_cpus::get();
    if number_of_bands < (number_of_cpus as u32) {
        println!("Notice: Performance could be increased because number of bands ({}) is lower than number of cpus ({})", number_of_bands, number_of_cpus);
    }

    println!("Using {} bands with height {}", number_of_bands, band_height);
}


fn ith_pixel_band(i: usize, pixel_chunk: &mut [u8], chunk_size: usize) -> PixelBand {
    PixelBand::new(pixel_chunk, i * chunk_size)
}


fn ith_window(i: usize, width: u32, band_height: u32, current_band_height: u32) -> Window {
    Window::new(
        0,
        (i as u32) * band_height,
        width,
        current_band_height,
    )
}


fn calculate_recursive(region: &Region, window: &Window, pixel_band: &mut PixelBand) {
    let unique_escape = unique_escape_for(region, window);

    if let Some(escape) = unique_escape {
        fill_window(region, window, escape, pixel_band);
        return;
    }

    let (part1, optional_part2) = window.split_if_sensible();

    if let Some(part2) = optional_part2 {
        calculate_recursive(region, &part1, pixel_band);
        calculate_recursive(region, &part2, pixel_band);
        return;
    }

    calculate_window(region, window, pixel_band);
}


fn unique_escape_for(region: &Region, window: &Window) -> Option<u8> {
    let first_pixel = Pixel { x: window.min_x, y: window.min_y };
    let first_point = region.point_for_pixel(&first_pixel);
    let unique_escape = first_point.escape_time(region.max_iterations);

    let window_border_pixels = WindowBorderIterator::new(window);

    for pixel in window_border_pixels {
        let point = region.point_for_pixel(&pixel);
        let escape = point.escape_time(region.max_iterations);

        if escape != unique_escape {
            return None;
        }
    }

    Some(unique_escape)
}


fn fill_window(region: &Region, window: &Window, color: u8, pixel_band: &mut PixelBand) {
    let window_area_pixels = WindowAreaIterator::new(window);

    for pixel in window_area_pixels {
        pixel_band.set_color_of_pixel(color, &pixel, region.width_in_pixels);
    }
}


fn calculate_window(region: &Region, window: &Window, pixel_band: &mut PixelBand) {
    let window_area_pixels = WindowAreaIterator::new(window);

    for pixel in window_area_pixels {
        let point = region.point_for_pixel(&pixel);
        let color = point.escape_time(region.max_iterations);

        pixel_band.set_color_of_pixel(color, &pixel, region.width_in_pixels);
    }
}
