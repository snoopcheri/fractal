use super::pixel::Pixel;


pub struct PixelBand<'a> {
    pixels: &'a mut [u8],
    offset: usize,
}


impl<'a> PixelBand<'a> {
    pub fn new(pixels: &'a mut [u8], offset: usize) -> Self {
        PixelBand { pixels, offset }
    }


    pub fn set_color_of_pixel(&mut self, color: u8, pixel: &Pixel, whole_display_width: u32) {
        let index = (pixel.y * whole_display_width + pixel.x) as usize;
        self.pixels[index - self.offset] = color;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_color_of_pixel_without_offset() {
        // arrange
        let mut pixels: Vec<u8> = vec!(0; 10 * 10);
        let pixel = Pixel { x: 7, y: 8 };

        // act
        {
            let mut pixel_band = PixelBand::new(&mut pixels, 0);
            pixel_band.set_color_of_pixel(42, &pixel, 10);
        }

        // assert
        assert_eq!(pixels[7 + 8 * 10], 42);
    }


    #[test]
    fn set_color_of_pixel_with_offset() {
        // arrange
        let mut pixels: Vec<u8> = vec!(0; 10 * 10);
        let pixel = Pixel { x: 7, y: 8 };

        // act
        {
            let mut chunks: Vec<&mut [u8]> = pixels.chunks_mut(10 * 5).collect();
            let mut pixel_band = PixelBand::new(&mut chunks[1], 10 * 5);
            pixel_band.set_color_of_pixel(42, &pixel, 10);
        }

        // assert
        assert_eq!(pixels[7 + 8 * 10], 42);
    }
}
