use super::pixel::Pixel;
use super::window::Window;


pub struct WindowLineIterator {
    max_x: u32,
    fixed_y: u32,
    current_x: u32,
}


impl WindowLineIterator {
    pub fn new(window: &Window, fixed_y: u32) -> Self {
        WindowLineIterator {
            max_x: window.min_x + window.width,
            fixed_y: fixed_y,
            current_x: window.min_x,
        }
    }
}


impl Iterator for WindowLineIterator {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x >= self.max_x {
            return None;
        }

        let pixel = Pixel {x: self.current_x, y: self.fixed_y};

        self.current_x += 1;

        Some(pixel)
    }
}


pub struct WindowAreaIterator {
    window: Window,
    current_x: u32,
    current_y: u32,
    pixels_left: u32,
}


impl WindowAreaIterator {
    pub fn new(window: &Window) -> Self {
        WindowAreaIterator {
            window: *window,
            current_x: window.min_x,
            current_y: window.min_y,
            pixels_left: window.width * window.height,
        }
    }
}


impl Iterator for WindowAreaIterator {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pixels_left == 0 {
            return None;
        }

        let pixel = Pixel { x: self.current_x, y: self.current_y };

        self.pixels_left -= 1;

        if self.pixels_left % self.window.width == 0 {
            self.current_x = self.window.min_x;
            self.current_y += 1;
        } else {
            self.current_x += 1;
        }

        Some(pixel)
    }
}


pub struct WindowBorderIterator {
    window: Window,
    total_pixels: u32,
    pixels_left: u32,
}


impl WindowBorderIterator {
    pub fn new(window: &Window) -> Self {
        let n_pixels = window.width * 2 + window.height * 2 - 4;

        WindowBorderIterator {
            window: *window,
            total_pixels: n_pixels,
            pixels_left: n_pixels,
        }
    }
}


impl Iterator for WindowBorderIterator {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pixels_left == 0 {
            return None;
        }

        self.pixels_left -= 1;

        let offset_x;
        let offset_y;

        if self.pixels_left >= self.total_pixels - self.window.width {
            offset_x = self.total_pixels - self.pixels_left -1;
            offset_y = 0;
        } else if self.pixels_left < self.window.width {
            offset_x = self.pixels_left;
            offset_y = self.window.height -1;
        } else {
            offset_x = (self.window.width - 1) * (self.pixels_left % 2);
            offset_y = (self.pixels_left - self.window.width) / 2 + 1;
        }

        Some(
            Pixel {
                x: self.window.min_x + offset_x,
                y: self.window.min_y + offset_y
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_line_iterator() {
        let window = Window::new(10, 20, 6, 4);
        let iterator = WindowLineIterator::new(&window, 21);

        // TODO: replace with proper assertion
//        for (idx, pixel) in iterator.enumerate() {
//            println!("{}: {:?}", idx, pixel);
//        }
    }

    #[test]
    fn window_area_iterator() {
        let window = Window::new(10, 20, 6, 4);
        let iterator = WindowAreaIterator::new(&window);

        // TODO: replace with proper assertion
//        for (idx, pixel) in iterator.enumerate() {
//            println!("{}: {:?}", idx, pixel);
//        }
    }

    #[test]
    fn window_border_iterator() {
        let window = Window::new(10, 20, 6, 4);
        let iterator = WindowBorderIterator::new(&window);

        // TODO: replace with proper assertion
//        for (idx, pixel) in iterator.enumerate() {
//            println!("{}: {:?}", idx, pixel);
//        }
    }
}
