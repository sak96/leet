//! Solution for https://leetcode.com/problems/rotate-function
//! 396. Rotate Function

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut value = 0;
        let mut total = 0;
        let n = nums.len() as i32;
        for (i, v) in nums.iter().enumerate() {
            value += (i as i32) * v;
            total += v;
        }
        let mut max_value = value;
        for i in nums.iter().rev() {
            value += total - (i * n);
            max_value = max_value.max(value)
        }
        max_value
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,3,2,6], 26)]
    #[case(vec![100], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_rotate_function(nums);
        assert_eq!(actual, expected);
    }
}
