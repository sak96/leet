#[derive(Default)]
pub struct MyHashSet (std::collections::HashSet<i32>);

impl MyHashSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, key: i32) {
        self.0.insert(key);
    }

    pub fn remove(&mut self, key: i32) {
        self.0.remove(&key);
    }

    pub fn contains(&self, key: i32) -> bool {
        self.0.contains(&key)
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
    }
}
