use super::mandelbrot::Mandelbrot;


pub trait MandelbrotEngine {
    fn calculate(&self, mandelbrot: &Mandelbrot, pixels: &mut Vec<u8>) {
        if self.should_calculate_in_parallel() {
            self.calculate_in_parallel(mandelbrot, pixels);
        } else {
            self.calculate_serially(mandelbrot, pixels);
        }
    }


    fn should_calculate_in_parallel(&self) -> bool;
    fn calculate_serially(&self, mandelbrot: &Mandelbrot, pixels: &mut Vec<u8>);
    fn calculate_in_parallel(&self, mandelbrot: &Mandelbrot, pixels: &mut Vec<u8>);
}
