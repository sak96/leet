//! Solution for https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i
//! 3191. Minimum Operations to Make Binary Array Elements Equal to One I

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..(nums.len() - 2) {
            if nums[i] == 0 {
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
                count += 1
            }
        }
        if nums[nums.len() - 2] == 0 && nums[nums.len() - 1] == 0 {
            -1
        } else {
            count
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
    #[case(vec![0,1,1,1,0,0], 3)]
    #[case(vec![0,1,1,1], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_operations(nums);
        assert_eq!(actual, expected);
    }
}
