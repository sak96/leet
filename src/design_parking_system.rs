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
        assert_eq!(car_park.add_car(1), true, "space present for big");
        assert_eq!(car_park.add_car(2), true, "space present for medium");
        assert_eq!(car_park.add_car(3), false, "no space for small");
        assert_eq!(car_park.add_car(1), false, "all space parked for big");
    }
}
