use std::cmp::Reverse;
use std::collections::BinaryHeap;
pub struct KthLargest {
    nums: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, mut nums: Vec<i32>) -> Self {
        let k = k as usize;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        nums.truncate(k);
        if nums.len() < k {
            nums.push(i32::MIN);
        }
        assert_eq!(nums.len(), k, "there will be at least k elements");
        Self {
            nums: nums.into_iter().map(Reverse).collect(),
        }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if Some(val) > self.nums.peek().as_ref().map(|x| x.0) {
            self.nums.pop();
            self.nums.push(Reverse(val));
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
