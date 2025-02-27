//! Solution for https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray
//! 1749. Maximum Absolute Sum of Any Subarray

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut local_max_sum = 0;
        let mut min_sum = 0;
        let mut local_min_sum = 0;
        for &i in &nums {
            local_max_sum = i.max(local_max_sum + i);
            max_sum = max_sum.max(local_max_sum);
            local_min_sum = i.min(local_min_sum + i);
            min_sum = min_sum.min(local_min_sum);
        }
        max_sum.max(min_sum.abs())
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,-3,2,3,-4], 5)]
    #[case(vec![2,-5,1,-4,3,-2], 8)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_absolute_sum(nums);
        assert_eq!(actual, expected);
    }
}
