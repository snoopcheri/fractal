use num::complex::Complex64;


#[derive(Debug)]
pub enum RegionType {
    Default,
    SeaHorseValley,
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Region {
    min_re: f64,
    min_im: f64,
    width: f64,
    height: f64,
}

impl Region {
    pub fn new(min_re: f64, max_re: f64, min_im: f64, max_im: f64) -> Region {
        let width = max_re - min_re;
        let height = max_im - min_im;

        Region { min_re, min_im, width, height }
    }

    pub fn new_for_center(center: Complex64, radius: f64) -> Region {
        Region::new(
            center.re - radius,
            center.re + radius,
            center.im - radius,
            center.im + radius,
        )
    }

    pub fn new_for_type(region_type: RegionType) -> Region {
        match region_type {
            RegionType::Default => Region::new(-2.0, 1.0, -1.0, 1.0),
            RegionType::SeaHorseValley => Region::new_for_center(Complex64::new(-0.74548, 0.11669), 0.01276)
        }
    }

    pub fn min_re(&self) -> f64 {
        self.min_re
    }

    pub fn min_im(&self) -> f64 {
        self.min_im
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_region() {
        // act
        let region = Region::new(-2.0, 2.0, -1.0, 1.0);

        // assert
        assert_eq!(region.min_re, -2.0);
        assert_eq!(region.min_im, -1.0);
        assert_eq!(region.region_width, 4.0);
        assert_eq!(region.region_height, 2.0);
    }
}
