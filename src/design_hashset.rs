pub struct MyHashSet(Box<[bool]>);

impl MyHashSet {
    const MAX_LENGTH: usize = 1_000_001;

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(vec![false; Self::MAX_LENGTH].into_boxed_slice())
    }

    pub fn add(&mut self, key: i32) {
        debug_assert!((key as usize) < Self::MAX_LENGTH);
        self.0[key as usize] = true;
    }

    pub fn remove(&mut self, key: i32) {
        debug_assert!((key as usize) < Self::MAX_LENGTH);
        self.0[key as usize] = false;
    }

    pub fn contains(&self, key: i32) -> bool {
        debug_assert!((key as usize) < Self::MAX_LENGTH);
        self.0[key as usize]
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
