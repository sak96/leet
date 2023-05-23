#![allow(dead_code)]
use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct KthLargest {
    nums: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let k = k as usize;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        nums.truncate(k);
        Self {
            nums: nums.into_iter().map(Reverse).collect(),
            k,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        if Some(val) > self.nums.peek().as_ref().map(|x| x.0) || self.nums.len() != self.k {
            self.nums.push(Reverse(val));
            if self.nums.len() > self.k {
                self.nums.pop();
            }
        }
        self.nums.peek().unwrap().0
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

    #[test]
    fn failed_one() {
        let mut kth_largest = KthLargest::new(2, [0].to_vec());
        assert_eq!(kth_largest.add(-1), -1);
        assert_eq!(kth_largest.add(1), 0);
        assert_eq!(kth_largest.add(-2), 0);
        assert_eq!(kth_largest.add(-4), 0);
        assert_eq!(kth_largest.add(3), 1);
    }
}
