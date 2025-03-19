//! Solution for https://leetcode.com/problems/max-consecutive-ones-iii
//! 1004. Max Consecutive Ones III

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let l = nums.len();
        let mut windows = std::collections::VecDeque::with_capacity(k + 1);
        let mut max = k;

        windows.push_front(0);
        for (i, n) in nums.into_iter().enumerate() {
            if n == 0 {
                if windows.len() == k + 1 {
                    let start = windows.pop_front().unwrap();
                    max = max.max(i - start);
                }
                windows.push_back(i + 1);
            }
        }
        max.max(l - windows.pop_front().unwrap()) as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,1,1,0,0,0,1,1,1,1,0], 2, 6)]
    #[case(vec![0,0,0,1], 4, 4)]
    #[case(vec![1,0,0,0], 2, 3)]
    #[case(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3, 10)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::longest_ones(nums, k);
        assert_eq!(actual, expected);
    }
}
