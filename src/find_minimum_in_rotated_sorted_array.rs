//! Solution for https://leetcode.com/problems/find-minimum-in-rotated-sorted-array
//! 153. Find Minimum in Rotated Sorted Array

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let last = nums.last().expect("1 <= n; n == nums.length");
        let part = nums.partition_point(|x| x > last);
        nums[part]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,4,5,1,2], 1)]
    #[case(vec![4,5,6,7,0,1,2],0)]
    #[case(vec![11,13,15,17], 11)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::find_min(nums);
        assert_eq!(actual, expected);
    }
}
