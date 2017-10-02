use num::complex::Complex64;


pub trait EscapeTime {
    fn escape_time(&self, max_iterations: u8) -> u8;
}


impl EscapeTime for Complex64 {
    fn escape_time(&self, max_iterations: u8) -> u8 {
        let mut point = *self;

        for escape in 1..max_iterations {
            point = point * point + self;

            if point.norm_sqr() > 4.0 {
                return escape;
            }
        }

        max_iterations
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_time_for_already_escaped_c_returns_one() {
        // arrange
        let c = Complex64::new(3.0, 0.0);

        // act
        let escape = c.escape_time(200);

        // assert
        assert_eq!(escape, 1);
    }

    #[test]
    fn escape_time_for_c_escaping_in_one_step_returns_one() {
        // arrange
        let c = Complex64::new(1.9, 0.0);

        // act
        let escape = c.escape_time(200);

        // assert
        assert_eq!(escape, 1);
    }

    #[test]
    fn escape_time_for_origin_returns_max_iterations() {
        // arrange
        let c = Complex64::new(0.0, 0.0);

        // act
        let escape = c.escape_time(200);

        // assert
        assert_eq!(escape, 200);
    }
}
