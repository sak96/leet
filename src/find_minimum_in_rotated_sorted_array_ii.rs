//! Solution for https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii
//! 154. Find Minimum in Rotated Sorted Array II

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let last = nums.last().expect("nums.length >= 1");
        let pos = nums.iter().position(|x| x != last).unwrap_or_default();
        let idx = nums[pos..].partition_point(|x| x > last);
        nums[(idx + pos).min(nums.len() - 1)].min(nums[0])
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3], 1)]
    #[case(vec![3,1,3, 3], 1)]
    #[case(vec![1,3,5], 1)]
    #[case(vec![2,2,2,0,1], 0)]
    #[case(vec![1,2,2,1,1], 1)]
    #[case(vec![1,1,2,1,1], 1)]
    #[case(vec![2,2,2,1,2], 1)]
    #[case(vec![2,2,2,2,2], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::find_min(nums);
        assert_eq!(actual, expected);
    }
}
