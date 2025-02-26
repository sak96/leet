pub struct ParkingSystem([i32; 3]);

impl ParkingSystem {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        Self([big, medium, small])
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
        let space = &mut self.0[car_type as usize - 1];
        if *space > 0 {
            *space -= 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn leet1() {
        let mut car_park = ParkingSystem::new(1, 1, 0);
        assert!(car_park.add_car(1), "space present for big");
        assert!(car_park.add_car(2), "space present for medium");
        assert!(!car_park.add_car(3), "no space for small");
        assert!(!car_park.add_car(1), "all space parked for big");
    }
}
