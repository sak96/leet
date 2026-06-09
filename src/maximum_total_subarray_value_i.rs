//! Solution for https://leetcode.com/problems/maximum-total-subarray-value-i
//! 3689. Maximum Total Subarray Value I

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let min = nums.iter().min().expect("1 <= nums.length");
        let max = nums.iter().max().expect("1 <= nums.length");
        (max - min) as i64 * k as i64
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,2], 2, todo!("Expected Result"))]
    #[case(vec![4,2,5,1], 3, todo!("Expected Result"))]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::max_total_value(nums, k);
        assert_eq!(actual, expected);
    }
}
