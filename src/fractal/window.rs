const MINIMUM_WINDOW_SIZE: u32 = 32;


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Window {
    pub min_x: u32,
    pub min_y: u32,
    pub width: u32,
    pub height: u32,
}


impl Window {
    pub fn new(min_x: u32, min_y: u32, width: u32, height: u32) -> Window {
        Window { min_x, min_y, width, height }
    }

    pub fn split_if_sensible(&self) -> (Window, Option<Window>) {
        // Splitting horizontally?
        if self.width > MINIMUM_WINDOW_SIZE && self.width >= self.height {
            let (left, right) =  self.horizontally_split();
            return (left, Some(right));
        }

        // Splitting Vertically?
        if self.height > MINIMUM_WINDOW_SIZE && self.height >= self.width {
            let (upper, lower) = self.vertically_split();
            return (upper, Some(lower));
        }

        // Don't split at all
        (*self, None)
    }

    fn horizontally_split(self) -> (Window, Window) {
        let left_part = Window {
            width: self.width / 2,
            ..self
        };

        let right_part = Window {
            min_x: left_part.min_x + left_part.width,
            width: self.width - left_part.width,
            ..self
        };

        (left_part, right_part)
    }

    pub fn vertically_split(self) -> (Window, Window) {
        let upper_part = Window {
            height: self.height / 2,
            ..self
        };

        let lower_part = Window {
            min_y: upper_part.min_y + upper_part.height,
            height: self.height - upper_part.height,
            ..self
        };

        (upper_part, lower_part)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_window() {
        // act
        let window = Window::new(10, 20, 30, 40);

        // assert
        assert_eq!(window.min_x, 10);
        assert_eq!(window.min_y, 20);
        assert_eq!(window.width, 30);
        assert_eq!(window.height, 40);
    }

    #[test]
    fn split_if_sensible_for_wide_window_returns_two_windows() {
        // arrange
        let window = Window { min_x: 10, min_y: 20, width: 100, height: 10};

        // act
        let (left_part, right_part) = window.split_if_sensible();

        // assert
        assert!(right_part.is_some());
        assert_eq!(left_part, Window { min_x: 10, min_y: 20, width: 50, height: 10});
        assert_eq!(right_part.unwrap(), Window { min_x: 60, min_y: 20, width: 50, height: 10});
    }

    #[test]
    fn split_if_sensible_for_tall_window_returns_two_windows() {
        // arrange
        let window = Window { min_x: 10, min_y: 20, width: 10, height: 100};

        // act
        let (upper_part, lower_part) = window.split_if_sensible();

        // assert
        assert!(lower_part.is_some());
        assert_eq!(upper_part, Window { min_x: 10, min_y: 20, width: 10, height: 50});
        assert_eq!(lower_part.unwrap(), Window { min_x: 10, min_y: 70, width: 10, height: 50});
    }

    #[test]
    fn split_if_sensible_for_small_window_returns_window() {
        // arrange
        let window = Window { min_x: 10, min_y: 20, width: 10, height: 10 };

        // act
        let (single_part, non_existing_part) = window.split_if_sensible();

        // assert
        assert!(non_existing_part.is_none());
        assert_eq!(single_part, window);
    }
}
