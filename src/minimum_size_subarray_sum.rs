//! Solution for https://leetcode.com/problems/minimum-size-subarray-sum
//! 209. Minimum Size Subarray Sum

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut left, mut sum, mut len) = (0, 0, nums.len() + 1);
        for right in 0..nums.len() {
            sum += nums[right];
            while sum >= target {
                len = len.min(right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }
        if len > nums.len() {
            0
        } else {
            len as _
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(7, vec![2,3,1,2,4,3], 2)]
    #[case(4, vec![1,4,4], 1)]
    #[case(11, vec![1,1,1,1,1,1,1,1], 0)]
    fn case(#[case] target: i32, #[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_sub_array_len(target, nums);
        assert_eq!(actual, expected);
    }
}
