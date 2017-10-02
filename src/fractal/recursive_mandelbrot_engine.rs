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

        calculate_recursive(region, &window, 0, pixels);
    }


    fn calculate_in_parallel(&self, region: &Region, pixels: &mut Vec<u8>) {
        verify_band_height_for_region(self.band_height, region);

        let pixel_bands: Vec<(usize, &mut [u8])> = pixels
            .chunks_mut((region.width_in_pixels * self.band_height) as usize)
            .enumerate()
            .collect();

        pixel_bands.into_par_iter()
            .for_each(|(band_nr, pixel_band)| {
                let band_nr = band_nr as u32;
                let current_band_height = (pixel_band.len() as u32) / region.width_in_pixels;
                let window_band = window_for_band(region, self.band_height, band_nr, current_band_height);
                let offset_in_pixels_slice = band_nr * self.band_height * region.width_in_pixels;

                calculate_recursive(region, &window_band, offset_in_pixels_slice, pixel_band);
            });
    }
}


fn verify_band_height_for_region(band_height: u32, region: &Region) {
    let number_of_bands = ((region.height_in_pixels as f64) / (band_height as f64)).ceil() as u32;
    assert!(number_of_bands > 0);

    let last_band_height = region.height_in_pixels - (number_of_bands-1) * band_height;
    assert!(last_band_height >= 8);

    let number_of_cpus = num_cpus::get();
    if number_of_bands < (number_of_cpus as u32) {
        println!("Notice: Performance could be increased because number of bands ({}) is lower than number of cpus ({})", number_of_bands, number_of_cpus);
    }

    println!("Using {} bands with height {}", number_of_bands, band_height);
}


fn window_for_band(region: &Region, band_height: u32, band_nr: u32, current_band_height: u32) -> Window {
    Window::new(
        0,
        band_nr * band_height,
        region.width_in_pixels,
        current_band_height
    )
}


fn calculate_recursive(region: &Region, window: &Window, offset_in_pixels_slice: u32, pixels: &mut [u8]) {
    let unique_escape = unique_escape_for(region, window);

    if let Some(escape) = unique_escape {
        fill_window(region, window, escape, offset_in_pixels_slice, pixels);
        return;
    }

    let (part1, optional_part2) = window.split_if_sensible();

    if let Some(part2) = optional_part2 {
        calculate_recursive(region, &part1, offset_in_pixels_slice, pixels);
        calculate_recursive(region, &part2, offset_in_pixels_slice, pixels);
        return;
    }

    calculate_window(region, window, offset_in_pixels_slice, pixels);
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


fn fill_window(region: &Region, window: &Window, escape: u8, offset_in_pixels_slice: u32, pixels: &mut [u8]) {
    let window_area_pixels = WindowAreaIterator::new(window);

    for pixel in window_area_pixels {
        pixel.draw(escape, region.width_in_pixels, offset_in_pixels_slice, pixels);
    }
}


fn calculate_window(region: &Region, window: &Window, offset_in_pixels_slice: u32, pixels: &mut [u8]) {
    let window_area_pixels = WindowAreaIterator::new(window);

    for pixel in window_area_pixels {
        let point = region.point_for_pixel(&pixel);
        let escape = point.escape_time(region.max_iterations);

        pixel.draw(escape, region.width_in_pixels, offset_in_pixels_slice, pixels);
    }
}
