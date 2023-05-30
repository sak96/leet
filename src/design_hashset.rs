#[derive(Default)]
pub struct MyHashSet(Vec<i32>);

impl MyHashSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, key: i32) {
        if let Err(idx) = self.0.binary_search(&key) {
            self.0.insert(idx, key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        if let Ok(idx) = self.0.binary_search(&key) {
            self.0.remove(idx);
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        self.0.binary_search(&key).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leet1() {
        let mut my_hashset = MyHashSet::new();
        my_hashset.add(1);
        my_hashset.add(2);
        assert!(my_hashset.contains(1));
        assert!(!my_hashset.contains(3), "not found");
        my_hashset.add(2);
        assert!(my_hashset.contains(2));
        my_hashset.remove(2);
        assert!(!my_hashset.contains(2), "already removed");
        my_hashset.add(1_000_000);
        assert!(my_hashset.contains(1_000_000));
        my_hashset.remove(1_000_000);
        assert!(!my_hashset.contains(1_000_000), "already removed");
    }
}
