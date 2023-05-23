#![allow(dead_code)]
struct KthLargest {
    nums: Vec<i32>,
    last: usize,
}

impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let k = k as usize;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        nums.truncate(k);
        Self { nums, last: k - 1 }
    }

    fn add(&mut self, val: i32) -> i32 {
        if Some(&val) > self.nums.get(self.last) {
            let idx = self.nums.partition_point(|&x| x > val);
            self.nums.insert(idx, val);
            self.nums.truncate(self.last + 1);
        }
        self.nums[self.last]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_case() {
        let mut kth_largest = KthLargest::new(3, [4, 5, 8, 2].to_vec());
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
    }

    #[test]
    fn start_with_k_minus_one() {
        let mut kth_largest = KthLargest::new(1, [].to_vec());
        assert_eq!(kth_largest.add(-3), -3);
        assert_eq!(kth_largest.add(-2), -2);
        assert_eq!(kth_largest.add(-4), -2);
        assert_eq!(kth_largest.add(0), 0);
        assert_eq!(kth_largest.add(4), 4);
    }
}
