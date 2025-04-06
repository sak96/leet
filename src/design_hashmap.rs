//! Solution for https://leetcode.com/problems/design-hashmap
//! 706. Design HashMap

pub struct MyHashMap(Vec<i32>);
// pub struct MyHashMap([i32; 1_000_000 + 1]);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    pub fn new() -> Self {
        Self(vec![-1; 1_000_000 + 1])
    }
    pub fn put(&mut self, key: i32, value: i32) {
        self.0[key as usize] = value
    }

    pub fn get(&self, key: i32) -> i32 {
        self.0[key as usize]
    }

    pub fn remove(&mut self, key: i32) {
        self.0[key as usize] = -1
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn leet_1() {
        let mut map = MyHashMap::new();
        map.put(1, 1); // The map is now [[1,1]]
        map.put(2, 2); // The map is now [[1,1], [2,2]]
        assert_eq!(map.get(1), 1); // return 1, The map is now [[1,1], [2,2]]
        assert_eq!(map.get(3), -1); // return -1 (i.e., not found), The map is now [[1,1], [2,2]]
        map.put(2, 1); // The map is now [[1,1], [2,1]] (i.e., update the existing value)
        assert_eq!(map.get(2), 1); // return 1, The map is now [[1,1], [2,1]]
        map.remove(2); // remove the mapping for 2, The map is now [[1,1]]
        assert_eq!(map.get(2), -1); // return -1 (i.e., not found), The map is now [[1,1]]
    }
}
