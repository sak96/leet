//! Solution for https://leetcode.com/problems/check-if-array-is-sorted-and-rotated
//! 1752. Check if Array Is Sorted and Rotated

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut idx = 0;
        while idx + 1 < nums.len() && nums[idx] <= nums[idx + 1] {
            idx += 1
        }
        if idx + 1 < nums.len() {
            if nums[nums.len() - 1] <= nums[0] {
                idx += 1;
                while idx + 1 < nums.len() && nums[idx] <= nums[idx + 1] {
                    idx += 1
                }
                idx + 1 == nums.len()
            } else {
                false
            }
        } else {
            true
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
    #[case(vec![3,4,5,1,2], true)]
    #[case(vec![3,3,4,5,1,1,2,2], true)]
    #[case(vec![3,3,4,5,4,4], false)]
    #[case(vec![3,3,4,5,4,4], false)]
    #[case(vec![2,1,3,4], false)]
    #[case(vec![1,2,3], true)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::check(nums);
        assert_eq!(actual, expected);
    }
}
