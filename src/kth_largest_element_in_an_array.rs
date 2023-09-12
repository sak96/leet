//! Solution for https://leetcode.com/problems/kth-largest-element-in-an-array
//! 215. Kth Largest Element in an Array

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums[nums.len() - k as usize]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,2,1,5,6,4], 2, 5)]
    #[case(vec![3,2,3,1,2,4,5,5,6], 4, 4)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::find_kth_largest(nums, k);
        assert_eq!(actual, expected);
    }
}
