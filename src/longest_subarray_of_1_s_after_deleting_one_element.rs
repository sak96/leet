//! Solution for https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element
//! 1493. Longest Subarray of 1's After Deleting One Element

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut longest_subarry_len = 0;
        let mut current_sub_array = 0;
        let mut sub_arry_zero_split = 0;
        for num in nums {
            if num == 0 {
                longest_subarry_len = longest_subarry_len.max(current_sub_array);
                current_sub_array = current_sub_array - sub_arry_zero_split + 1;
                sub_arry_zero_split = current_sub_array;
            } else {
                current_sub_array += 1;
            }
        }
        (longest_subarry_len.max(current_sub_array) - 1) as _
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,1,0,1], 3)]
    #[case(vec![0,1,1,1,0,1,1,0,1], 5)]
    #[case(vec![1,1,1], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::longest_subarray(nums);
        assert_eq!(actual, expected);
    }
}
