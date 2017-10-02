use super::region::Region;


pub trait MandelbrotEngine {
    fn calculate(&self, region: &Region, pixels: &mut Vec<u8>) {
        if self.should_calculate_in_parallel() {
            self.calculate_in_parallel(region, pixels);
        } else {
            self.calculate_serially(region, pixels);
        }
    }


    fn should_calculate_in_parallel(&self) -> bool;
    fn calculate_serially(&self, region: &Region, pixels: &mut Vec<u8>);
    fn calculate_in_parallel(&self, region: &Region, pixels: &mut Vec<u8>);
}
