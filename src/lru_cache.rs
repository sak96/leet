//! Solution for https://leetcode.com/problems/lru-cache
//! 146. LRU Cache

use std::collections::HashMap;
pub struct Node {
    value: i32,
    prev: Option<i32>,
    next: Option<i32>,
}
pub struct LRUCache {
    cache: HashMap<i32, Node>,
    first: Option<i32>,
    last: Option<i32>,
    capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            cache: HashMap::with_capacity(capacity),
            first: None,
            last: None,
            capacity,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.remove_(key) {
            self.put(key, value);
            value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.remove_(key);
        if self.cache.len() == self.capacity {
            self.remove_(self.first.unwrap());
        }
        self.cache.insert(
            key,
            Node {
                value,
                prev: self.last,
                next: None,
            },
        );
        match self.last {
            None => self.first = Some(key),
            Some(k) => self.cache.get_mut(&k).unwrap().next = Some(key),
        }
        self.last = Some(key);
    }

    fn remove_(&mut self, key: i32) -> Option<i32> {
        if let Some(node) = self.cache.remove(&key) {
            match node.prev {
                Some(prev) => self.cache.get_mut(&prev).unwrap().next = node.next,
                None => self.first = node.next,
            }
            match node.next {
                Some(next) => self.cache.get_mut(&next).unwrap().prev = node.prev,
                None => self.last = node.prev,
            }
            Some(node.value)
        } else {
            None
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leet1() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1); // cache is {1=1}
        cache.put(2, 2); // cache is {1=1, 2=2}
        assert_eq!(cache.get(1), 1); // return 1
        cache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        assert_eq!(cache.get(2), -1); // returns -1 (not found)
        cache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        assert_eq!(cache.get(1), -1); // return -1 (not found)
        assert_eq!(cache.get(3), 3); // return 3
        assert_eq!(cache.get(4), 4); // return 4
    }
}
