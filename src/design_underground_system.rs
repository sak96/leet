use std::collections::HashMap;
#[derive(Default)]
pub struct UndergroundSystem {
    checked_in: HashMap<i32, (String, i32)>,
    stats: HashMap<(String, String), (i32, i32)>,
}

impl UndergroundSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check_in(&mut self, id: i32, src: String, t: i32) {
        self.checked_in.insert(id, (src, t));
    }

    pub fn check_out(&mut self, id: i32, dest: String, t2: i32) {
        let (src, t1) = self.checked_in.remove(&id).unwrap();
        let (sum, count) = &mut self.stats.entry((src, dest)).or_default();
        *sum += t2 - t1;
        *count += 1;
    }

    pub fn get_average_time(&self, src: String, dest: String) -> f64 {
        self.stats
            .get(&(src, dest))
            .map(|&(sum, count)| sum as f64 / count as f64)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;

    #[test]
    pub fn leet1() {
        let mut underground_system = UndergroundSystem::new();
        underground_system.check_in(45, "Leyton".to_string(), 3);
        underground_system.check_in(32, "Paradise".to_string(), 8);
        underground_system.check_in(27, "Leyton".to_string(), 10);
        underground_system.check_out(45, "Waterloo".to_string(), 15);
        underground_system.check_out(27, "Waterloo".to_string(), 20);
        underground_system.check_out(32, "Cambridge".to_string(), 22);
        assert_float_eq!(
            underground_system.get_average_time("Paradise".to_string(), "Cambridge".to_string()),
            14.00000
        );
        assert_float_eq!(
            underground_system.get_average_time("Paradise".to_string(), "Cambridge".to_string()),
            14.00000
        );
        assert_float_eq!(
            underground_system.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            11.00000
        );

        underground_system.check_in(10, "Leyton".to_string(), 24);
        assert_float_eq!(
            underground_system.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            11.00000
        );
        underground_system.check_out(10, "Waterloo".to_string(), 38);
        assert_eq!(
            underground_system.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            12.00000
        );
    }

    #[test]
    pub fn leet2() {
        let mut underground_system = UndergroundSystem::new();
        underground_system.check_in(10, "Leyton".to_string(), 3);
        underground_system.check_out(10, "Paradise".to_string(), 8);
        assert_float_eq!(
            underground_system.get_average_time("Leyton".to_string(), "Paradise".to_string()),
            5.00000
        );
        underground_system.check_in(5, "Leyton".to_string(), 10);
        underground_system.check_out(5, "Paradise".to_string(), 16);
        underground_system.get_average_time("Leyton".to_string(), "Paradise".to_string());
        underground_system.check_in(2, "Leyton".to_string(), 21);
        underground_system.check_out(2, "Paradise".to_string(), 30);
        assert_float_eq!(
            underground_system.get_average_time("Leyton".to_string(), "Paradise".to_string()),
            6.66667
        );
    }
}
