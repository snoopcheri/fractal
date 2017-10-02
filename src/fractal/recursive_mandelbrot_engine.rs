use rayon::prelude::*;
use num_cpus;

use super::mandelbrot_engine::MandelbrotEngine;
use super::region::Region;
use super::window::Window;
use super::pixel::Pixel;
use super::window_iterator::WindowAreaIterator;
use super::window_iterator::WindowBorderIterator;
use super::escape_time::EscapeTime;


pub struct RecursiveMandelbrotEngine {
    in_parallel: bool,
}

impl RecursiveMandelbrotEngine {
    pub fn new(in_parallel: bool) -> RecursiveMandelbrotEngine {
        RecursiveMandelbrotEngine { in_parallel }
    }
}


impl MandelbrotEngine for RecursiveMandelbrotEngine {
    fn should_calculate_in_parallel(&self) -> bool {
        self.in_parallel
    }


    fn calculate_serially(&self, region: &Region, pixels: &mut Vec<u8>) {
        let window = Window::new(0, 0, region.width_in_pixels, region.height_in_pixels);

        calculate_recursive(region, &window, 0, pixels);
    }


    fn calculate_in_parallel(&self, region: &Region, pixels: &mut Vec<u8>) {
        let number_of_bands = num_cpus::get() as u32;
        assert_eq!(region.height_in_pixels % number_of_bands, 0);

        let number_of_rows_per_band = region.height_in_pixels / number_of_bands;

        let pixel_bands: Vec<(usize, &mut [u8])> = pixels
            .chunks_mut((region.width_in_pixels * number_of_rows_per_band) as usize)
            .enumerate()
            .collect();

        pixel_bands.into_par_iter()
            .for_each(|(band_nr, pixel_band)| {
                let band_nr = band_nr as u32;
                let window_band = window_for_band(region, band_nr, number_of_rows_per_band);
                let pixel_offset = band_nr * number_of_rows_per_band * region.width_in_pixels;

                calculate_recursive(region, &window_band, pixel_offset, pixel_band);
            });
    }
}


fn window_for_band(region: &Region, band_nr: u32, number_of_rows_per_band: u32) -> Window {
    Window::new(
        0,
        band_nr * number_of_rows_per_band,
        region.width_in_pixels,
        number_of_rows_per_band
    )
}


fn calculate_recursive(region: &Region, window: &Window, pixel_offset: u32, pixels: &mut [u8]) {
    let unique_escape = unique_escape_for(region, window);

    if let Some(escape) = unique_escape {
        fill_window(region, window, escape, pixel_offset, pixels);
        return;
    }

    let (part1, optional_part2) = window.split_if_sensible();

    if let Some(part2) = optional_part2 {
        calculate_recursive(region, &part1, pixel_offset, pixels);
        calculate_recursive(region, &part2, pixel_offset, pixels);
        return;
    }

    calculate_window(region, window, pixel_offset, pixels);
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


fn fill_window(region: &Region, window: &Window, escape: u8, pixel_offset: u32, pixels: &mut [u8]) {
    let window_area_pixels = WindowAreaIterator::new(window);

    for pixel in window_area_pixels {
        pixel.draw(escape, region.width_in_pixels, pixel_offset, pixels);
    }
}


fn calculate_window(region: &Region, window: &Window, pixel_offset: u32, pixels: &mut [u8]) {
    let window_area_pixels = WindowAreaIterator::new(window);

    for pixel in window_area_pixels {
        let point = region.point_for_pixel(&pixel);
        let escape = point.escape_time(region.max_iterations);

        pixel.draw(escape, region.width_in_pixels, pixel_offset, pixels);
    }
}
